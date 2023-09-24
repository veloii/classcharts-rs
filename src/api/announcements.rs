use crate::{
    api::helpers::deserialize_yes_no_bool,
    client::{ErrorResponse, CCParser, SuccessResponse, Client},
};
use serde::Deserialize;
use serde_json::Value;

use super::helpers::Empty;

#[derive(Deserialize, Debug)]
pub struct Attachment {
    pub filename: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Announcement {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub school_name: String,
    pub teacher_name: String,
    pub school_logo: Option<String>,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub sticky: bool,
    pub state: Option<String>,
    pub timestamp: String,
    pub attachments: Vec<Attachment>,
    pub for_pupils: Vec<Value>,
    pub comment_visibility: String,

    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub allow_comments: bool,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub allow_reactions: bool,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub allow_consent: bool,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub priority_pinned: bool,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub requires_consent: bool,

    pub can_change_consent: bool,
    pub consent: Option<Value>,
    pub pupil_consents: Vec<Value>,
}

pub type AnnouncementsMeta = Vec<Empty>;
pub type AnnouncementsData = Vec<Announcement>;

pub type Announcements = SuccessResponse<AnnouncementsData, AnnouncementsMeta>;

impl Client {
    /// Gets the current student's announcements 
    pub async fn get_announcements(&mut self) -> Result<Announcements, ErrorResponse> {
        let request = self
            .build_get(format!("/announcements/{}", self.student_id))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Announcements = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_announcements_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let announcements_response = server.mock(|when, then| {
            when.method(GET)
                .path("/apiv2student/announcements/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                      "success": 1,
                      "data": [
                        {
                            "id": 2384823,
                            "title": "Title",
                            "description": "<p>Description</p>",
                            "school_name": "School Name",
                            "teacher_name": "Teacher name",
                            "school_logo": "https://example.com",
                            "sticky": "yes",
                            "state": "viewed",
                            "timestamp": "2023-02-23T10:00:00+00:00",
                            "attachments": [],
                            "for_pupils": [],
                            "comment_visibility": "none",
                            "allow_comments": "no",
                            "allow_reactions": "no",
                            "allow_consent": "no",
                            "priority_pinned": "no",
                            "requires_consent": "no",
                            "can_change_consent": false,
                            "consent": null,
                            "pupil_consents": []
                        }
                      ],
                      "meta": []
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_announcements().await.unwrap();

        announcements_response.assert();
    }
}
