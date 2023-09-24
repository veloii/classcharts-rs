#![recursion_limit = "256"]

mod client;
mod macros;

pub use client::ApiRequestError as ErrorResponse;
pub use client::CCResponse as SuccessResponse;
pub use client::Client;
pub use client::ClientCreationError as ClientError;
pub mod api;
