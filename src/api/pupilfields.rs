use serde::Deserialize;

use crate::client::{ErrorResponse, CCParser, SuccessResponse, Client};

use super::helpers::Empty;

#[derive(Deserialize, Debug)]
pub struct PupilField {
    pub id: usize,
    pub name: String,
    pub graphic: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct PupilFieldsData {
    pub note: String,
    pub fields: Vec<PupilField>,
}

pub type PupilFieldsMeta = Vec<Empty>;

pub type PupilFields = SuccessResponse<PupilFieldsData, PupilFieldsMeta>;

impl Client {
    /*
    * Gets the current student's pupil fields 
    */
    pub async fn get_pupilfields(&mut self) -> Result<PupilFields, ErrorResponse> {
        let request = self
            .build_get(format!("/customfields/{}", self.student_id))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: PupilFields = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn get_pupilfields_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let pupilfields_response = server.mock(|when, then| {
            when.method(GET)
                .path("/apiv2student/customfields/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 1,
                    "data": {
                        "note": "Note",
                        "fields": [
                            {
                                "id": 43583485,
                                "name": "Field Name",
                                "graphic": "#000",
                                "value": "Value",
                            }
                        ]
                    },
                    "meta": [],
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_pupilfields().await.unwrap();

        pupilfields_response.assert();
    }
}
