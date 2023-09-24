use std::collections::HashMap;

use crate::client::{ErrorResponse, CCParser, SuccessResponse, Client};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct BehaviourStyle {
    pub border_color: Option<String>,
    pub custom_class: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BehaviourTimelinePoint {
    pub positive: usize,
    pub negative: isize,
    pub name: String,
    pub start: String,
    pub end: String,
}

#[derive(Deserialize, Debug)]
pub struct BehaviourData {
    pub timeline: Vec<BehaviourTimelinePoint>,
    #[serde(deserialize_with = "deserialize_hashmap_or_empty_array")]
    pub positive_reasons: HashMap<String, usize>,
    #[serde(deserialize_with = "deserialize_hashmap_or_empty_array")]
    pub negative_reasons: HashMap<String, usize>,
    pub other_positive: Vec<String>,
    pub other_negative: Vec<String>,
    pub other_positive_count: Vec<HashMap<String, usize>>,
    pub other_negative_count: Vec<HashMap<String, usize>>,
}

fn deserialize_hashmap_or_empty_array<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, usize>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize the JSON data into a Value first
    let value: Value = Deserialize::deserialize(deserializer)?;

    // Check if it's an empty array
    if value.is_array() && value.as_array().unwrap().is_empty() {
        return Ok(HashMap::new());
    }

    // Check if it's a hashmap
    if value.is_object() {
        return Ok(serde_json::from_value(value).map_err(serde::de::Error::custom)?);
    }

    Err(serde::de::Error::custom("Invalid JSON format for 'data'"))
}

#[derive(Deserialize, Debug)]
pub struct BehaviourMeta {
    pub start_date: String,
    pub end_date: String,
    pub step_size: String,
}

pub type Behaviour = SuccessResponse<BehaviourData, BehaviourMeta>;

pub struct BehaviourOptions {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl Client {
    /*
    * Gets the current student's behaviour 
    */
    pub async fn get_behaviour(
        &mut self,
        options: Option<BehaviourOptions>,
    ) -> Result<Behaviour, ErrorResponse> {
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
            .build_get(format!("/behaviour/{}?{}", self.student_id, params))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Behaviour = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_behaviour_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let behaviour_response = server.mock(|when, then| {
            when.method(GET).path("/apiv2student/behaviour/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                        "success": 1,
                        "data": {
                            "timeline": [
                                {
                                    "positive": 4,
                                    "negative": 0,
                                    "name": "9/4/2023",
                                    "start": "2023-09-04",
                                    "end": "2023-09-10"
                                },
                                {
                                    "positive": 3,
                                    "negative": 0,
                                    "name": "9/11/2023",
                                    "start": "2023-09-11",
                                    "end": "2023-09-17"
                                },
                                {
                                    "positive": 4,
                                    "negative": 0,
                                    "name": "9/18/2023",
                                    "start": "2023-09-18",
                                    "end": "2023-09-24"
                                }
                            ],
                            "positive_reasons": {
                                "Test 1": 4,
                                "Test 2": 3,
                                "Test 3": 4,
                            },
                            "negative_reasons": [],
                            "other_positive": [],
                            "other_negative": [],
                            "other_positive_count": [],
                            "other_negative_count": []
                        },
                        "meta": {
                            "start_date": "2023-09-04T00:00:00+00:00",
                            "end_date": "2023-09-24T23:59:59+00:00",
                            "step_size": "week"
                        },
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_behaviour(None).await.unwrap();

        behaviour_response.assert();
    }
}
