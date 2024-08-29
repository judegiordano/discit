use axum::response::Response;

use crate::errors::AppError;

pub type ApiResponse = Result<Response, AppError>;

pub const ONE_SECOND_MS: i64 = 1_000;
pub const ONE_MINUTE_MS: i64 = ONE_SECOND_MS * 60;
pub const ONE_HOUR_MS: i64 = ONE_MINUTE_MS * 60;
pub const ONE_DAY_MS: i64 = ONE_HOUR_MS * 24;
