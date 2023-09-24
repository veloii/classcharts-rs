pub mod activity;
pub mod announcements;
pub mod attendance;
pub mod badges;
pub mod behaviour;
pub mod detentions;
mod helpers;
pub mod homework;
pub mod lessons;
pub mod pupilfields;
pub mod rewards;
pub mod student;

#[cfg(test)]
mod tests {
    use crate::Client;

    impl Client {
        pub fn generate_mock(base_url: String) -> Client {
            return Client::manual_creation(
                "student_id".to_string(),
                base_url,
                "auth_cookies".to_string(),
                "session_id".to_string(),
            );
        }
    }
}
