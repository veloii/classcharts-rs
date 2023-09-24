use serde::Deserialize;

use crate::client::{ErrorResponse, CCParser, SuccessResponse, Client};

use super::helpers::Empty;

#[derive(Deserialize, Debug)]
pub struct Teacher {
    pub title: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Debug)]
pub struct LessonPupilBehaviour {
    pub reason: String,
    pub score: usize,
    pub polarity: String,
    pub timestamp: String,
    pub teacher: Teacher,
}

#[derive(Deserialize, Debug)]
pub struct PupilEvent {
    pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct PupilBadge {
    pub timestamp: String,
    pub lesson_pupil_behaviour: LessonPupilBehaviour,
    pub event: PupilEvent,
}

#[derive(Deserialize, Debug)]
pub struct Badge {
    pub id: usize,
    pub name: String,
    pub icon: String,
    pub colour: String,
    pub created_date: String,
    pub pupil_badges: Vec<PupilBadge>,
    pub icon_url: String,
}

pub type BadgesData = Vec<Badge>;
pub type BadgesMeta = Vec<Empty>;
pub type Badges = SuccessResponse<BadgesData, BadgesMeta>;

impl Client {
    /// Gets the current student's earned badges 
    pub async fn get_badges(&mut self) -> Result<Badges, ErrorResponse> {
        let request = self
            .build_get(format!("/eventbadges/{}", self.student_id))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Badges = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_badges_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();
        // Create a mock on the server.
        let badges_response = server.mock(|when, then| {
            when.method(GET)
                .path("/apiv2student/eventbadges/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 1,
                    "data": [
                        {
                            "id": 59345,
                            "name": "Test",
                            "icon": "https://example.com",
                            "colour": "#000",
                            "created_date": "2023-08-25T00:00:00+00:00",
                            "pupil_badges": [{
                                "timestamp": "2023-08-25T00:00:00+00:00",
                                "lesson_pupil_behaviour": {
                                    "reason": "Test",
                                    "score": 10,
                                    "polarity": "positive",
                                    "timestamp": "2023-08-25T00:00:00+00:00",
                                    "teacher": {
                                        "title": "Mr",
                                        "first_name": "first_name",
                                        "last_name": "last_name",
                                    },
                                },
                                "event": {
                                    "label": "Test",
                                },
                            }],
                            "icon_url": "https://example.com",
                        }
                    ],
                    "meta": [],
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_badges().await.unwrap();

        badges_response.assert();
    }
}
