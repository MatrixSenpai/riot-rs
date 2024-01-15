use serde::Deserialize;

pub type ApiResult<T> = Result<T, ApiError>;
pub type EmptyApiResult = ApiResult<()>;

#[derive(Debug)]
pub enum ApiError {
    RiotApiError(RiotApiErrorMessage, reqwest::StatusCode),
    ReqwestError(reqwest::Error),
}

#[derive(Deserialize, Debug)]
pub struct RiotApiErrorStatus {
    pub status: RiotApiErrorMessage,
}

#[derive(Deserialize, Debug)]
pub struct RiotApiErrorMessage {
    status_code: usize,
    message: String,
}

#[derive(Debug)]
pub enum ConversionError {
    InvalidStringError,
    MissingDataError,
}

pub fn map_reqwest_error(e: reqwest::Error) -> ApiError { ApiError::ReqwestError(e) }