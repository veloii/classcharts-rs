use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{borrow::Cow, string::FromUtf8Error};
use thiserror::Error;

use reqwest::{header::ToStrError, IntoUrl, RequestBuilder, Response};

use crate::{api::student::StudentInfoData, new_params};

#[derive(Debug)]
pub struct Client {
    pub session_id: String,
    pub student_id: String,
    reqwest_client: reqwest::Client,
    base_url: String,
    auth_cookies: String,
    last_session_id_updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct SessionCookie {
    session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct CCStatusResponse {
    pub error: Option<String>,
    pub success: usize,
}

#[derive(Deserialize, Debug)]
pub struct SuccessResponse<Data, Meta> {
    pub data: Data,
    pub meta: Meta,
}

#[derive(Error, Debug)]
pub enum ClientCreationError {
    #[error(
        "Unauthenticated, either your code or date of birth is wrong. No cookies were provided."
    )]
    AuthenticationError,

    #[error("Failed to send the API request or create the reqwest client")]
    ClientError(#[from] reqwest::Error),

    #[error("Cookie cannot not be parsed")]
    CookieParsingError(#[from] serde_json::Error),

    #[error("Session cookie does not exist on server returned cookies")]
    MissingSesssionCookie(()),

    #[error("Could not parse header as a string")]
    StringParseError(#[from] ToStrError),

    #[error("Failed to get student info, error: {0}")]
    ApiRequestError(#[from] ErrorResponse),

    #[error("Failed to decode the cookie")]
    StringDecodingError(#[from] FromUtf8Error),
}

#[async_trait]
pub trait CCParser {
    async fn cc_parse(self) -> Result<String, ErrorResponse>;
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorResponse {
    #[error("Failed to process the request")]
    GenericClientError(#[from] reqwest::Error),

    #[error("Failed to parse the text response")]
    TextParsingError(#[source] reqwest::Error),

    #[error("Could not parse the json response")]
    SerdeJsonParsingError(#[from] serde_json::Error),

    #[error("ClassCharts returned the error code: {0}")]
    ClassChartsStatusError(usize),

    #[error("ClassCharts returned the error code: {0} and message {1}")]
    ClassChartsError(usize, String),
}

#[derive(Deserialize, Debug)]
pub struct SessionMeta {
    pub session_id: String,
}

pub type Session = SuccessResponse<StudentInfoData, SessionMeta>;

#[async_trait]
impl CCParser for Response {
    async fn cc_parse(mut self) -> Result<String, ErrorResponse> {
        let text = self
            .text()
            .await
            .map_err(ErrorResponse::TextParsingError)?;

        let json = serde_json::from_str::<CCStatusResponse>(&text)?;

        if json.success != 1 {
            if let Some(error) = json.error {
                return Err(ErrorResponse::ClassChartsError(json.success, error));
            } else {
                return Err(ErrorResponse::ClassChartsStatusError(json.success));
            }
        }

        return Ok(text);
    }
}

impl Client {
    pub async fn build_get<P>(&mut self, path: P) -> Result<RequestBuilder, ErrorResponse>
    where
        P: IntoUrl + std::fmt::Display,
    {
        if (self.last_session_id_updated.time() - Utc::now().time()).num_minutes() > 3 {
            self.get_new_session_id().await?;
        }

        return Ok(self
            .reqwest_client
            .get(format!("{}/apiv2student{}", self.base_url, path))
            .header("Cookie", &self.auth_cookies)
            .header("Authorization", format!("Basic {}", self.session_id)));
    }

    pub async fn build_post<P>(&mut self, path: P) -> Result<RequestBuilder, ErrorResponse>
    where
        P: IntoUrl + std::fmt::Display,
    {
        if (self.last_session_id_updated.time() - Utc::now().time()).num_minutes() > 3 {
            self.get_new_session_id().await?;
        }

        return Ok(self
            .reqwest_client
            .post(format!("{}/apiv2student{}", self.base_url, path))
            .header("Cookie", &self.auth_cookies)
            .header("Authorization", format!("Basic {}", self.session_id)));
    }

    pub async fn get_new_session_id(&mut self) -> Result<String, ErrorResponse> {
        let params = new_params!("include_data", "true");

        let request = self
            .reqwest_client
            .post(format!("{}/apiv2student/ping", self.base_url))
            .header("Cookie", &self.auth_cookies)
            .header("Authorization", format!("Basic {}", self.session_id))
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(params)
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Session = serde_json::from_str(&text)?;

        let session_id = data.meta.session_id;

        self.session_id = session_id.clone();
        self.last_session_id_updated = Utc::now();

        return Ok(session_id);
    }

    pub fn manual_creation(
        student_id: String,
        base_url: String,
        auth_cookies: String,
        session_id: String,
    ) -> Client {
        return Client {
            student_id,
            base_url,
            reqwest_client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            last_session_id_updated: Utc::now(),
            auth_cookies,
            session_id,
        };
    }

    pub async fn create<C, D>(
        code: C,
        dob: D,
        base_url: Option<String>,
    ) -> Result<Self, ClientCreationError>
    where
        C: ToString,
        D: Into<Cow<'static, str>>,
    {
        let reqwest_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let base_url = base_url.unwrap_or("https://www.classcharts.com".to_string());

        let login_form = reqwest::multipart::Form::new()
            .text("_method", "POST")
            .text("code", code.to_string().to_uppercase())
            .text("dob", dob)
            .text("remember_me", "1")
            .text("recaptcha-token", "no-token-available");

        let login_request = reqwest_client
            .post(format!("{}/student/login", base_url))
            .multipart(login_form)
            .send()
            .await?;

        let mut cookies = login_request.cookies();
        let headers = login_request.headers();
        let status = login_request.status();

        if status != 302 || headers.get("set-cookie").is_none() {
            return Err(ClientCreationError::AuthenticationError);
        }

        let session_cookie = cookies
            .find(|cookie| cookie.name() == "student_session_credentials")
            .ok_or(())
            .map_err(ClientCreationError::MissingSesssionCookie)?;

        // i don't think we actually need this
        let session_cookie = urlencoding::decode(session_cookie.value())?;

        let session_id = serde_json::from_str::<SessionCookie>(&session_cookie)?.session_id;

        let auth_cookies = headers
            .get("set-cookie")
            .unwrap()
            .to_str()?
            .split(",")
            .collect::<Vec<&str>>()
            .join(";");

        let mut client = Client {
            session_id,
            student_id: "".to_string(),
            reqwest_client,
            base_url,
            auth_cookies,
            last_session_id_updated: Utc::now(),
        };

        let cc_response = client
            .get_student_info()
            .await
            .map_err(ClientCreationError::ApiRequestError)?;

        client.student_id = cc_response.data.user.id.to_string();

        return Ok(client);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn cc_parser_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let error_mock = server.mock(|when, then| {
            when.method(GET).path("/error");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 0,
                    "error": "test error"
                }));
        });

        let success_mock = server.mock(|when, then| {
            when.method(GET).path("/success");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 1,
                    "data": "success",
                    "meta": [],
                }));
        });

        let client = reqwest::Client::new();

        let error_request = client
            .get(format!("{}/error", server.base_url()))
            .send()
            .await
            .unwrap();
        let success_request = client
            .get(format!("{}/success", server.base_url()))
            .send()
            .await
            .unwrap();

        error_request.cc_parse().await.unwrap_err();
        success_request.cc_parse().await.unwrap();

        success_mock.assert();
        error_mock.assert();
    }

    #[tokio::test]
    async fn create_client_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let student_login_response = server.mock(|when, then| {
            when.method(POST).path("/student/login");
            then.status(302)
                .header("content-type", "application/json")
                .header(
                    "set-cookie",
                    "student_session_credentials={\"session_id\":\"jf99rm23pdi29dj32fh23i\"}",
                );
        });

        let student_info_response = server.mock(|when, then| {
            when.method(POST).path("/apiv2student/ping");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 1,
                    "data": {
                        "user": {
                            "id": 3949234,
                            "name": "Name",
                            "first_name": "first_name",
                            "last_name": "last_name",
                            "avatar_url": "https://example.com",
                            "display_behaviour": false,
                            "display_parent_behaviour": false,
                            "display_homework": false,
                            "display_rewards": false,
                            "display_detentions": false,
                            "display_report_cards": false,
                            "display_classes": false,
                            "display_announcements": true,
                            "display_academic_reports": false,
                            "display_attendance": true,
                            "display_attendance_type": "instance",
                            "display_attendance_percentage": false,
                            "display_activity": false,
                            "display_mental_health": false,
                            "display_mental_health_no_tracker": false,
                            "display_timetable": false,
                            "is_disabled": false,
                            "display_two_way_communications": true,
                            "display_absences": false,
                            "can_upload_attachments": false,
                            "display_event_badges": false,
                            "display_avatars": false,
                            "display_concern_submission": false,
                            "display_custom_fields": false,
                            "pupil_concerns_help_text": "",
                            "allow_pupils_add_timetable_notes": false,
                            "detention_alias_plural_uc": "Detentions",
                            "announcements_count": 0,
                            "messages_count": 0,
                            "pusher_channel_name": "pusher_channel_name",
                            "has_birthday": false,
                            "has_new_survey": false,
                            "survey_id": null
                        }
                    },
                    "meta": {
                        "session_id": "jf99rm23pdi29dj32fh23i",
                        "version": "27.16.2",
                    },
                }));
        });

        let client = Client::create("my_code", "my_dob", Some(server.base_url()))
            .await
            .unwrap();

        assert_eq!(client.student_id, "3949234");
        assert_eq!(client.session_id, "jf99rm23pdi29dj32fh23i");

        student_login_response.assert();
        student_info_response.assert();
    }
}
