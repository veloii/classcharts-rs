use crate::{
    api::helpers::deserialize_yes_no_bool,
    client::{ApiRequestError, CCParser, CCResponse, Client},
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum DetentionAttended {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "upscaled")]
    Upscaled,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Deserialize, Debug)]
pub struct School {
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub opt_notes_names: bool,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub opt_notes_comments: bool,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub opt_notes_comments_pupils: bool,
}

#[derive(Deserialize, Debug)]
pub struct Pupil {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
    pub school: School,
}

#[derive(Deserialize, Debug)]
pub struct Subject {
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Lesson {
    pub id: usize,
    pub name: String,
    pub subject: Subject,
}

#[derive(Deserialize, Debug)]
pub struct LessonPupilBehaviour {
    pub reason: String,
}

#[derive(Deserialize, Debug)]
pub struct Teacher {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct DetentionType {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Detention {
    pub id: usize,
    pub attended: DetentionAttended,
    pub date: Option<String>,
    pub length: Option<usize>,
    pub location: Option<String>,
    pub notes: Option<String>,
    pub time: Option<String>,
    pub pupil: Pupil,
    pub lesson: Option<Lesson>,
    pub lesson_pupil_behaviour: LessonPupilBehaviour,
    pub teacher: Option<Teacher>,
    pub detention_type: DetentionType,
}

pub type DetentionsData = Vec<Detention>;

#[derive(Deserialize, Debug)]
pub struct DetentionsMeta {
    pub detention_alias_plural: String,
}

pub type Detentions = CCResponse<DetentionsData, DetentionsMeta>;

impl Client {
    pub async fn get_detentions(&mut self) -> Result<Detentions, ApiRequestError> {
        let request = self
            .build_get(format!("/detentions/{}", self.student_id))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Detentions = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_detentions_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let detentions_response = server.mock(|when, then| {
            when.method(GET).path("/apiv2student/detentions/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                        "success": 1,
                        "data": [
                            {
                                "id": 345834959,
                                "attended": "yes",
                                "date": "2023-08-24:00:00+00:00",
                                "length": 10,
                                "location": "Location",
                                "notes": "Notes",
                                "time": "16:00",
                                "pupil": {
                                    "id": 34593945,
                                    "first_name": "first_name",
                                    "last_name": "last_name",
                                    "school": {
                                        "opt_notes_names": "yes",
                                        "opt_notes_comments": "no",
                                        "opt_notes_comments_pupils": "no"
                                    }
                                },
                                "lesson": {
                                    "id": 4503459,
                                    "name": "Lesson Name",
                                    "subject": {
                                        "id": 5496945,
                                        "name": "Subject Name",
                                    }
                                },
                                "lesson_pupil_behaviour": {
                                    "reason": "reason"
                                },
                                "teacher": {
                                    "id": 435345345,
                                    "title": "Mr",
                                    "first_name": "first_name",
                                    "last_name": "last_name",
                                },
                                "detention_type": {
                                    "name": "detention_type"
                                }
                            }
                        ],
                        "meta": {
                          "detention_alias_plural": "detentions"
                        }
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_detentions().await.unwrap();

        detentions_response.assert();
    }
}
