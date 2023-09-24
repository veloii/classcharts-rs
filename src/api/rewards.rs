use serde::Deserialize;
use serde_json::Value;

use crate::api::helpers::deserialize_yes_no_bool;
use crate::client::{ApiRequestError, CCParser, CCResponse, Client};

use super::helpers::Empty;

#[derive(Debug)]
pub enum PurchasedCount {
    String(String),
    Number(usize),
}

impl<'de> Deserialize<'de> for PurchasedCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::String(s) => Ok(PurchasedCount::String(s)),
            Value::Number(num) if num.is_u64() => {
                Ok(PurchasedCount::Number(num.as_u64().unwrap() as usize))
            }
            _ => Err(serde::de::Error::custom(
                "Invalid format for 'purchased_count' field",
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RewardItem {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub photo: String,
    pub price: usize,
    pub stock_control: bool,
    pub stock: usize,
    pub can_purchase: bool,
    pub unable_to_purchase_reason: String,
    pub once_per_pupil: bool,
    pub purchased: bool,
    pub purchased_count: PurchasedCount,
    pub price_balance_difference: usize,
}

#[derive(Deserialize, Debug)]
pub struct RewardsMeta {
    pub pupil_score_balance: usize,
}

pub type RewardsData = Vec<RewardItem>;

pub type Rewards = CCResponse<RewardsData, RewardsMeta>;

#[derive(Deserialize, Debug)]
pub struct RewardPurchaseData {
    #[serde(deserialize_with = "deserialize_yes_no_bool")]
    pub single_purchase: bool,
    pub order_id: usize,
    pub balance: usize,
}

pub type RewardPurchaseMeta = Vec<Empty>;

pub type RewardPurchase = CCResponse<RewardPurchaseData, RewardPurchaseMeta>;

impl Client {
    pub async fn get_rewards(&mut self) -> Result<Rewards, ApiRequestError> {
        let request = self
            .build_get(format!("/rewards/{}", self.student_id))
            .await?
            .send()
            .await?;

        let text = request.cc_parse().await?;
        let data: Rewards = serde_json::from_str(&text)?;

        return Ok(data);
    }

    pub async fn purchase_reward<T>(
        &mut self,
        item_id: T,
    ) -> Result<RewardPurchase, ApiRequestError>
    where
        T: std::fmt::Display,
    {
        let request = self
            .build_post(format!("/purchase/{}", item_id))
            .await?
            .body(format!("pupil_id={}", self.student_id))
            .send()
            .await?;

        let text = request.cc_parse().await.map_err(|err| {
            if let ApiRequestError::SerdeJsonParsingError(_err) = err {
                return ApiRequestError::ClassChartsError(
                    0,
                    "Internal Server Error, the item may not exist.".to_string(),
                );
            } else {
                return err;
            }
        })?;
        let data: RewardPurchase = serde_json::from_str(&text)?;

        return Ok(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn purchase_reward_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let rewards_response = server.mock(|when, then| {
            when.method(POST).path("/apiv2student/purchase/item_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 1,
                    "data": {
                        "order_id": 3458354,
                        "single_purchase": "yes",
                        "balance": 0,
                    },
                    "meta": [],
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.purchase_reward("item_id").await.unwrap();

        rewards_response.assert();
    }

    #[tokio::test]
    async fn get_rewards_test() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let rewards_response = server.mock(|when, then| {
            when.method(GET).path("/apiv2student/rewards/student_id");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "success": 1,
                    "data": [
                        {
                            "id": 84564,
                            "name": "Name",
                            "description": "Description",
                            "photo": "https://example.com",
                            "price": 10,
                            "stock_control": true,
                            "stock": 1,
                            "can_purchase": true,
                            "unable_to_purchase_reason": "",
                            "once_per_pupil": false,
                            "purchased": true,
                            "purchased_count": "10",
                            "price_balance_difference": 0
                        }
                    ],
                    "meta": {
                        "pupil_score_balance": 10,
                    },
                }));
        });

        let mut client = Client::generate_mock(server.base_url());

        let _ = client.get_rewards().await.unwrap();

        rewards_response.assert();
    }
}
