use chrono::NaiveDate;
use serde::Deserialize;

use crate::{
    client::{ErrorResponse, CCParser, SuccessResponse, Client},
    new_params,
};

#[derive(Deserialize, Debug)]
pub struct Lesson {
    pub teacher_name: String,
    pub lesson_name: String,
    pub subject_name: String,
    pub is_alternative_lesson: bool,
    pub period_name: String,
    pub period_number: String,
    pub room_name: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub key: usize,
    pub note_abstract: String,
    pub note: String,
    pub pupil_note_abstract: String,
    pub pupil_note: String,
    pub pupil_note_raw: String,
}

#[derive(Deserialize, Debug)]
pub struct LessonsMeta {
    pub dates: Vec<String>,
    pub timetable_dates: Vec<String>,
    pub start_time: String,
    pub end_time: String,
}

pub type LessonsData = Vec<Lesson>;

pub type Lessons = SuccessResponse<LessonsData, LessonsMeta>;

impl Client {
    /// Gets the current student's lessons for a given date.
    /// This is using `chrono` for parsing the date.
    /// 
    /// Example:
    /// ```ignore
    /// // Gets the student's lessons for the current day. 
    /// client.get_lessons(chrono::Utc::now().date());
    /// ```
    pub async fn get_lessons(&mut self, date: NaiveDate) -> Result<Lessons, ErrorResponse> {
        let params = new_params!("date", &date.format("%Y-%m-%d").to_string()); 

        let request = self
            .build_get(format!("/timetable/{}?{}", self.student_id, params))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Lessons = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_lessons_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let lessons_response = server.mock(|when, then| {
            when.method(GET).path("/apiv2student/timetable/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                        "success": 1,
                        "data": [
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 34599345,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P1",
                                "period_number": "P1",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T08:45:00+01:00",
                                "end_time": "2023-09-26T09:45:00+01:00",
                                "key": 349593459,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 34953945,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P2",
                                "period_number": "P2",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T09:45:00+01:00",
                                "end_time": "2023-09-26T10:45:00+01:00",
                                "key": 3459345,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 9348345,
                                "lesson_name": "Lesson Name",
                                "subject_name": "TUTOR",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:Tut",
                                "period_number": "Tut",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T10:45:00+01:00",
                                "end_time": "2023-09-26T11:05:00+01:00",
                                "key": 43858345,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 34959345,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P3",
                                "period_number": "P3",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T11:20:00+01:00",
                                "end_time": "2023-09-26T12:20:00+01:00",
                                "key": 348583458,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 6325347,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P3",
                                "period_number": "P3",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T11:20:00+01:00",
                                "end_time": "2023-09-26T12:20:00+01:00",
                                "key": 348583458,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 34959345,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P4",
                                "period_number": "P4",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T13:05:00+01:00",
                                "end_time": "2023-09-26T14:05:00+01:00",
                                "key": 34858345,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 34858345,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P5",
                                "period_number": "P5",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T14:05:00+01:00",
                                "end_time": "2023-09-26T15:05:00+01:00",
                                "key": 438584358,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            },
                            {
                                "teacher_name": "Teacher Name",
                                "lesson_id": 4855834,
                                "lesson_name": "Lesson Name",
                                "subject_name": "Subject Name",
                                "is_alternative_lesson": false,
                                "period_name": "2Tue:P6",
                                "period_number": "P6",
                                "room_name": "Room Name",
                                "date": "2023-09-26",
                                "start_time": "2023-09-26T15:05:00+01:00",
                                "end_time": "2023-09-26T16:00:00+01:00",
                                "key": 3485435,
                                "note_abstract": "",
                                "note": "",
                                "pupil_note_abstract": "",
                                "pupil_note": "",
                                "pupil_note_raw": ""
                            }
                        ],
                        "meta": {
                            "dates": [
                                "2023-09-26"
                            ],
                            "timetable_dates": [
                                "2023-09-26",
                            ],
                            "periods": [
                                {
                                    "number": "P1",
                                    "start_time": "08:45:00",
                                    "end_time": "09:45:00"
                                },
                                {
                                    "number": "P2",
                                    "start_time": "09:45:00",
                                    "end_time": "10:45:00"
                                },
                                {
                                    "number": "Tut",
                                    "start_time": "10:45:00",
                                    "end_time": "11:05:00"
                                },
                                {
                                    "number": "P3",
                                    "start_time": "11:20:00",
                                    "end_time": "12:20:00"
                                },
                                {
                                    "number": "P4",
                                    "start_time": "13:05:00",
                                    "end_time": "14:05:00"
                                },
                                {
                                    "number": "P5",
                                    "start_time": "14:05:00",
                                    "end_time": "15:05:00"
                                },
                                {
                                    "number": "P6",
                                    "start_time": "15:05:00",
                                    "end_time": "16:00:00"
                                }
                            ],
                            "start_time": "2023-09-26T00:00:00+00:00",
                            "end_time": "2023-09-26T23:59:59+00:00"
                        }
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client
            .get_lessons(NaiveDate::from_ymd_opt(2023, 9, 26).unwrap())
            .await
            .unwrap();

        lessons_response.assert();
    }
}
