#![recursion_limit = "256"]

//! # An unoffical ClassCharts Student API Library
//!
//! [Repository](https://github.com/veloii/classcharts-rs)
//!
//! The student version of ClassCharts is allows students to view their homework, timetable,
//! attendance, etc. This library aims to help people create applications to interact with the
//! ClassCharts API.
//!
//! ## Prerequisites
//!
//! * A ClassCharts Access Code (provided by your school). This is NOT saved or sent to anywhere
//! but ClassChart's servers.
//!
//! ## Usage
//!
//! To create a ClassCharts Student Client and get their info.
//!
//! ```rust,no_run
//! use classcharts::Client;
//! # #[tokio::main]
//! # async fn main() {
//! let mut client = Client::create("your access code", "your date of birth
//! (DD/MM/YYYY)", None).await.unwrap();
//!
//! let student_info = client.get_student_info().await.unwrap();
//! println!("{:?}", student_info);
//! # }
//! ```
//!
//! To view the current student's homework:
//!
//! ```rust,no_run
//! # use classcharts::Client;
//! # #[tokio::main]
//! # async fn main() {
//! # let mut client = Client::create("your access code", "your date of birth
//! # (DD/MM/YYYY)", None).await.unwrap();
//! let homework = client.get_homeworks(None).await.unwrap();
//! # }
//! ```
//!
//! For a complete list of ClassCharts methods the `Client` exposes:
//! * `get_activity`
//! * `get_full_activity`
//! * `get_announcements`
//! * `get_attendance`
//! * `get_badges`
//! * `get_behaviour`
//! * `get_detentions`
//! * `get_homeworks`
//! * `get_lessons`
//! * `get_pupilfields`
//! * `get_rewards`
//! * `purchase_reward`
//! * `get_student_info`
//!
//! They will all return a `Result<SuccessResponse, ErrorResponse>`.
//!
//! # Responses and Errors
//!
//! This library trys to not abstract over the ClassCharts API too much.
//!
//! ## `SuccessResponse<Data, Meta>` struct
//! 
//! This wraps the `Data` and `Meta` in a struct with their respective property names. This will be
//! emitted when the ClassCharts API returns `{ success: 1 }`.
//!
//! You can find the specfic `Data` / `Meta` under `classcharts::api`, for example
//! `classcharts::api::homework::HomeworkData`.
//!
//! ## `ErrorResponse` enum
//!
//! This will be either:
//! * `GenericClientError` - reqwest:Error 
//! * `TextParsingError` - reqwest:Error 
//! * `SerdeJsonParsingError` - serde_json::Error 
//! * `ClassChartsStatusError` - This will occur when the ClassCharts API returns a non `{ success: 1 }` with no error message attribute
//! * `ClassChartsError` - Similar to `ClassChartsStatusError`, but it includes the error message attribute ClassCharts returned

mod client;
mod macros;

pub use client::ErrorResponse;
pub use client::SuccessResponse;
pub use client::Client;
pub use client::ClientCreationError as ClientError;
pub mod api;
