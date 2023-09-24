use crate::api::helpers::deserialize_yes_no_bool;
use crate::client::{ErrorResponse, CCParser, SuccessResponse, Client};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub enum HomeworkState {
    #[serde(rename = "not_completed")]
    NotCompleted,
    #[serde(rename = "late")]
    Late,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Deserialize, Debug)]
pub struct HomeworkStatus {
    pub id: usize,
    pub state: Option<HomeworkState>,
    pub mark: Value,
    pub mark_relative: usize,
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub ticked: bool,
    pub allow_attachments: bool,
    pub first_seen_date: Option<String>,
    pub last_seen_date: Option<String>,
    pub attachments: Vec<Value>,
    pub has_feedback: bool,
}

#[derive(Deserialize, Debug)]
pub struct ValidatedHomeworkAttachment {
    pub id: usize,
    pub file_name: String,
    pub file: String,
    pub validated_file: String,
}

#[derive(Deserialize, Debug)]
pub struct Homework {
    pub lesson: String,
    pub subject: String,
    pub teacher: String,
    pub homework_type: String,
    pub id: usize,
    pub title: String,
    pub meta_title: String,
    pub description: String,
    pub issue_date: String,
    pub due_date: String,
    pub completion_time_unit: String,
    pub completion_time_value: String,
    pub publish_time: String,
    pub status: HomeworkStatus,
    pub validated_links: Vec<Value>,
    pub validated_attachments: Vec<ValidatedHomeworkAttachment>,
}

#[derive(Deserialize, Debug)]
pub enum DisplayDate {
    #[serde(rename = "due_date")]
    DueDate,
    #[serde(rename = "issue_date")]
    IssueDate,
}

pub struct HomeworkOptions {
    pub display_date: Option<DisplayDate>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

#[derive(Deserialize, Debug)]
pub struct HomeworkMeta {
    pub start_date: String,
    pub end_date: String,
    pub display_type: DisplayDate,
    pub max_files_allowed: usize,
    pub allowed_file_types: Vec<String>,
    pub this_week_due_count: usize,
    pub this_week_outstanding_count: usize,
    pub this_week_completed_count: usize,
    pub allow_attachments: bool,
    pub display_marks: bool,
}

pub type HomeworkData = Vec<Homework>;

pub type Homeworks = SuccessResponse<HomeworkData, HomeworkMeta>;

impl Client {
    /// Gets the current student's homework 
    /// This is using `chrono` for parsing the date.
    /// 
    /// Example:
    /// ```ignore
    /// // Gets homework due in from the current day till the next day.
    /// client.get_homework(Some(
    ///     HomeworkOptions {
    ///         display_date: Some(DisplayDate::DueDate),
    ///         from: Some(chrono::Utc::now().date()),
    ///         to: Some(chrono::Utc::now().checked_add_days(chrono::Days(1)).date()),
    ///     }
    /// ));
    /// ```
    pub async fn get_homeworks(
        &mut self,
        options: Option<HomeworkOptions>,
    ) -> Result<Homeworks, ErrorResponse> {
        let mut params = url::form_urlencoded::Serializer::new(String::new());
 
        if let Some(options) = options {
            if let Some(to) = options.to {
                params.append_pair("to", &to.format("%Y-%m-%d").to_string());
            }
            if let Some(from) = options.from {
                params.append_pair("from", &from.format("%Y-%m-%d").to_string());
            }
        }

        let params = params.finish();

        let request = self
            .build_get(format!("/homeworks/{}?{}", self.student_id, params))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Homeworks = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_homeworks_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let homeworks_response = server.mock(|when, then| {
            when.method(GET).path("/apiv2student/homeworks/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                       "success": 1,
                        "data": [
                            {
                                "lesson": "Lesson",
                                "subject": "Subject",
                                "teacher": "Teacher",
                                "homework_type": "Homework",
                                "id": 5488456,
                                "title": "Maths Homework",
                                "meta_title": "",
                                "description": "<p>Description</p>",
                                "issue_date": "2023-09-16",
                                "due_date": "2023-09-19",
                                "completion_time_unit": "minutes",
                                "completion_time_value": "",
                                "publish_time": "00:00:00",
                                "status": {
                                    "id": 3459984,
                                    "state": "completed",
                                    "mark": null,
                                    "mark_relative": 0,
                                    "ticked": "yes",
                                    "allow_attachments": false,
                                    "allow_marking_completed": true,
                                    "first_seen_date": "2023-09-15T12:19:16+00:00",
                                    "last_seen_date": "2023-09-16T11:17:11+00:00",
                                    "attachments": [],
                                    "has_feedback": false
                                },
                                "validated_links": [],
                                "validated_attachments": []
                            }
                        ],
                        "meta": {
                            "start_date": "2023-09-17",
                            "end_date": "2023-10-24",
                            "display_type": "due_date",
                            "max_files_allowed": 5,
                            "allowed_file_types": [
                                "doc",
                                "docx",
                                "pdf",
                                "xls",
                                "xlsx",
                                "ppt",
                                "pptx",
                                "pub",
                                "txt",
                                "png",
                                "jpeg",
                                "jpg",
                                "gif",
                                "rtf",
                                "mp3",
                                "odt",
                                "odp",
                                "csv",
                                "mp4",
                                "mov",
                                "m4a",
                                "sb3",
                                "py"
                            ],
                            "this_week_due_count": 0,
                            "this_week_outstanding_count": 0,
                            "this_week_completed_count": 1,
                            "allow_attachments": true,
                            "display_marks": false
                        }
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_homeworks(None).await.unwrap();

        homeworks_response.assert();
    }
}
