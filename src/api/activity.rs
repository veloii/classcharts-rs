use crate::client::{ErrorResponse, CCParser, SuccessResponse, Client};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct ActivityStyle {
    pub border_color: Option<String>,
    pub custom_class: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ActivityPoint {
    pub id: usize,
    #[serde(rename = "type")]
    pub point_type: String,
    pub polarity: String,
    pub reason: String,
    pub score: isize,
    pub timestamp: String,
    pub timestamp_custom_time: Option<String>,
    pub style: ActivityStyle,
    pub pupil_name: String,
    pub lesson_name: Option<String>,
    pub teacher_name: String,
    pub room_name: Option<String>,
    pub note: Option<String>,
    pub _can_delete: bool,
    pub badges: Option<String>,
    pub detention_date: Option<String>,
    pub detention_time: Option<String>,
    pub detention_location: Option<String>,
    pub detention_type: Option<String>,
}

pub type ActivityData = Vec<ActivityPoint>;

#[derive(Debug)]
pub enum LastId {
    Boolean(bool),
    Number(usize),
}

impl<'de> Deserialize<'de> for LastId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::Bool(b) => Ok(LastId::Boolean(b)),
            Value::Number(num) if num.is_u64() => {
                Ok(LastId::Number(num.as_u64().unwrap() as usize))
            }
            _ => Err(serde::de::Error::custom(
                "Invalid format for 'last_id' field",
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ActivityMeta {
    pub start_date: String,
    pub end_date: String,
    pub step_size: String,
    pub last_id: Option<LastId>,
    pub detention_alias_uc: String,
}

pub type Activity = SuccessResponse<ActivityData, ActivityMeta>;

pub struct ActivityOptions {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub last_id: Option<String>,
}

pub struct FullActivityOptions {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

impl Client {
    /*
    * Gets the current student's activity 
    *
    * This function is only used for pagination, you likely want .get_full_activity */
    pub async fn get_activity(
        &mut self,
        options: Option<ActivityOptions>,
    ) -> Result<Activity, ErrorResponse> {
        let mut params = url::form_urlencoded::Serializer::new(String::new());

        if let Some(options) = options {
            if let Some(to) = options.to {
                params.append_pair("to", &to.format("%Y-%m-%d").to_string());
            }
            if let Some(from) = options.from {
                params.append_pair("from", &from.format("%Y-%m-%d").to_string());
            }
            if let Some(last_id) = options.last_id {
                params.append_pair("last_id", &last_id);
            }
        }

        let params = params.finish();

        let request = self
            .build_get(format!("/activity/{}?{}", self.student_id, params))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Activity = serde_json::from_str(&text)?;

        return Ok(data);
    }

    /*
    * Gets the current student's activity between two dates
    *
    * This function will automatically paginate through all the data returned by get_activity 
    */
    pub async fn get_full_activity(
        &mut self,
        options: FullActivityOptions,
    ) -> Result<ActivityData, ErrorResponse> {
        let mut data: ActivityData = vec![];
        let mut prev_last: Option<String> = None;

        loop {
            let params = ActivityOptions {
                from: Some(options.from),
                to: Some(options.to),
                last_id: prev_last.clone(),
            };

            let mut fragment = self.get_activity(Some(params)).await?.data;

            if fragment.is_empty() || fragment.len() == 0 {
                break;
            } else {
                prev_last = fragment.last().and_then(|item| Some(item.id.to_string()));
                data.append(&mut fragment);
            }
        }

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_activity_meta_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let activity_response = server
            .mock_async(|when, then| {
                when.method(GET).path("/apiv2student/activity/student_id");
                then.status(200)
                    .header("content-type", "application/json")
                    .json_body(json!({
                        "success": 1,
                        "data": [],
                        "meta": {
                            "start_date": "2023-08-24T23:00:00+00:00",
                            "end_date": "2023-09-24T22:59:59+00:00",
                            "last_id": false,
                            "step_size": "week",
                            "detention_alias_uc": "Detention"
                        }
                    }));
            })
            .await;

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_activity(None).await.unwrap();

        activity_response.assert_async().await;
    }
    #[tokio::test]
    async fn get_activity_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let activity_response = server
            .mock_async(|when, then| {
                when.method(GET).path("/apiv2student/activity/student_id");
                then.status(200)
                    .header("content-type", "application/json")
                    .json_body(json!({
                          "success": 1,
                          "data": [
                            {
                                "id": 3459349,
                                "type": "behaviour",
                                "polarity": "positive",
                                "reason": "Reason",
                                "score": 1,
                                "timestamp": "2023-04-21 10:00:00",
                                "timestamp_custom_time": null,
                                "style": {
                                    "border_color": null,
                                    "custom_class": null
                                },
                                "pupil_name": "Pupil Name",
                                "lesson_name": "Lesson Name",
                                "teacher_name": "Teacher Name",
                                "room_name": null,
                                "note": null,
                                "_can_delete": false,
                                "badges": "",
                                "detention_date": null,
                                "detention_time": null,
                                "detention_location": null,
                                "detention_type": null
                            }
                        ],
                        "meta": {
                            "start_date": "2023-08-24T23:00:00+00:00",
                            "end_date": "2023-09-24T22:59:59+00:00",
                            "last_id": 1,
                            "step_size": "week",
                            "detention_alias_uc": "Detention"
                        }
                    }));
            })
            .await;

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_activity(None).await.unwrap();

        activity_response.assert_async().await;
    }
}
