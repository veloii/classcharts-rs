use crate::{
    client::{ApiRequestError, CCParser, CCResponse, Client},
    new_params,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Student {
    pub id: usize,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: String,
    pub display_behaviour: bool,
    pub display_parent_behaviour: bool,
    pub display_homework: bool,
    pub display_rewards: bool,
    pub display_detentions: bool,
    pub display_report_cards: bool,
    pub display_classes: bool,
    pub display_announcements: bool,
    pub display_attendance: bool,
    pub display_attendance_type: String,
    pub display_attendance_percentage: bool,
    pub display_activity: bool,
    pub display_mental_health: bool,
    pub display_timetable: bool,
    pub is_disabled: bool,
    pub display_two_way_communications: bool,
    pub display_absences: bool,
    pub can_upload_attachments: bool,
    pub display_event_badges: bool,
    pub display_avatars: bool,
    pub display_concern_submission: bool,
    pub display_custom_fields: bool,
    pub pupil_concerns_help_text: String,
    pub allow_pupils_add_timetable_notes: bool,
    pub announcements_count: usize,
    pub messages_count: usize,
    pub pusher_channel_name: String,
    pub has_birthday: bool,
    pub has_new_survey: bool,
    pub survey_id: Option<usize>,
    pub detention_alias_plural_uc: String,
}

#[derive(Deserialize, Debug)]
pub struct StudentInfoData {
    pub user: Student,
}

#[derive(Deserialize, Debug)]
pub struct StudentInfoMeta {
    pub version: String,
}

pub type StudentInfo = CCResponse<StudentInfoData, StudentInfoMeta>;

impl Client {
    pub async fn get_student_info(&mut self) -> Result<StudentInfo, ApiRequestError> {
        let params = new_params!("include_data", "true");

        let request = self
            .build_post("/ping")
            .await?
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(params)
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: StudentInfo = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_student_info_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
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

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_student_info().await.unwrap();

        student_info_response.assert();
    }
}
