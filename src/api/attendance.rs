use crate::client::{ErrorResponse, CCParser, SuccessResponse, Client};
use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;

pub struct AttendanceOptions {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub enum AttendancePeriodStatus {
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "ignore")]
    Ignore,
}

#[derive(Debug)]
pub enum LateMinutes {
    String(String),
    Number(usize),
}

impl<'de> Deserialize<'de> for LateMinutes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::String(s) => Ok(LateMinutes::String(s)),
            Value::Number(num) if num.is_u64() => {
                Ok(LateMinutes::Number(num.as_u64().unwrap() as usize))
            }
            _ => Err(serde::de::Error::custom(
                "Invalid format for 'late_minutes' field",
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AttendancePeriod {
    pub code: String,
    pub status: AttendancePeriodStatus,
    pub late_minutes: LateMinutes,
    pub lesson_name: Option<String>,
    pub room_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AttendanceMeta {
    pub dates: Vec<String>,
    pub sessions: Vec<String>,
    pub start_date: String,
    pub percentage: String,
    pub percentage_singe_august: String,
}

pub type AttendanceData = HashMap<String, HashMap<String, AttendancePeriod>>;

pub type Attendance = SuccessResponse<AttendanceData, AttendanceMeta>;

impl Client {
    /*
    * Gets the current student's attendance 
    */
    pub async fn get_attendance(
        &mut self,
        options: Option<AttendanceOptions>,
    ) -> Result<Attendance, ErrorResponse> {
        let mut params = url::form_urlencoded::Serializer::new(String::new());

        if let Some(options) = options {
            params.append_pair("to", &options.to.format("%Y-%m-%d").to_string());
            params.append_pair("from", &options.from.format("%Y-%m-%d").to_string());
        }

        let params = params.finish();

        let request = self
            .build_get(format!("/attendance/{}?{}", self.student_id, params))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Attendance = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_attendance_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let attendance_response = server.mock(|when, then| {
            when.method(GET).path("/apiv2student/attendance/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                      "success": 1,
                      "data": {
                        "2023-08-25": {
                            "AM": {
                                "code": "#",
                                "status": "ignore",
                                "late_minutes": 0
                            },
                            "PM": {
                                "code": "#",
                                "status": "ignore",
                                "late_minutes": 0
                            },
                            "Period 1": {
                                "code": "",
                                "late_minutes": "",
                                "status": "ignore"
                            },
                            "Period 2": {
                                "code": "",
                                "late_minutes": "",
                                "status": "ignore"
                            },
                            "Period Tut": {
                                "code": "",
                                "late_minutes": "",
                                "status": "ignore"
                            },
                            "Period 3": {
                                "code": "",
                                "late_minutes": "",
                                "status": "ignore"
                            },
                            "Period 4": {
                                "code": "",
                                "late_minutes": "",
                                "status": "ignore"
                            },
                            "Period 5": {
                                "code": "",
                                "late_minutes": "",
                                "status": "ignore"
                            }
                        },
                      },
                      "meta": {
                        "dates": [
                            "2023-08-25",
                        ],
                        "sessions": [
                            "AM",
                            "PM",
                            "Period 1",
                            "Period 2",
                            "Period Tut",
                            "Period 3",
                            "Period 4",
                            "Period 5",
                            "Period 6",
                        ],
                        "start_date": "2023-08-25T00:00:00+00:00",
                        "end_date": "2023-08-25T16:00:00+00:00",
                        "percentage": "100",
                        "percentage_singe_august": "100"
                    }
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_attendance(None).await.unwrap();

        attendance_response.assert();
    }
}
