use reqwest::Response;
use serde::Deserialize;

use super::{api_helper, responses::ErrorResponse};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
  pub code: i32,
  pub message: String,
  pub user_facing_message: Option<String>,
}

// I have yet to see a request with multiple errors Im leaving it because the csrf refetch code uses it and i cant bother
type ApiErrors = Vec<ApiError>;

#[derive(Debug)]
pub enum OxidError {
  /// Typically 400 errors returned from roblox
  Api(ApiErrors),

  ///
  Http(reqwest::Error),

  /// A bad cookie is used to perform a request
  Unauthorized,

  /// errors from processing of data fetched from the API
  Validation(String),
}

impl std::fmt::Display for OxidError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OxidError::Api(errors) => write!(f, "OxidRoblox API Error: {:?}", errors),
      OxidError::Http(e) => write!(f, "OxidRoblox HTTP Error: {}", e),
      OxidError::Unauthorized => write!(f, "OxidRoblox Error: Invalid or unauthorized cookie when using an authenticated request"),
      OxidError::Validation(msg) => write!(f, "OxidRoblox Rrror: {}", msg),
    }
  }
}

impl std::error::Error for OxidError {}

/// parses the error json from a bad request and returns Ok(ApiErrors)
///
pub(crate) async fn get_api_errors_from_response(response: Response) -> RobloxResult<ApiErrors> {
  Ok(api_helper::deserialize_body::<ErrorResponse>(response).await?.errors)
}

pub type RobloxResult<T> = Result<T, OxidError>;
