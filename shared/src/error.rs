use async_graphql::{Error, ErrorExtensionValues, ErrorExtensions};
use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
  #[error("UnAuthorized")]
  UnAuthorized(&'static str),

  #[error("Server Error")]
  ExecutionError(String),

  #[error("Server Error")]
  DatabaseSeaError(String),

  #[error("Server Error")]
  DatabaseRecordNotFoundError {
    table: String,
    col: String,
    value: String,
  },
}

impl ErrorExtensions for AppError {
  fn extend(&self) -> Error {
    use AppError::*;

    Error::new(format!("{}", self)).extend_with(|_err, e| match self {
      UnAuthorized(reason) => specific_code_and_reason(StatusCode::UNAUTHORIZED, reason, e),

      ExecutionError(reason) => {
        specific_code_and_reason(StatusCode::INTERNAL_SERVER_ERROR, reason, e)
      }

      DatabaseSeaError(reason) => {
        specific_code_and_reason(StatusCode::INTERNAL_SERVER_ERROR, reason, e)
      }

      DatabaseRecordNotFoundError { col, table, value } => specific_code_and_reason(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!(
          "DatabaseRecordNotFoundError reason: Not found record on table {} with {} is {}",
          table, col, value
        )
        .as_str(),
        e,
      ),
    })
  }
}

fn specific_code_and_reason(code: StatusCode, reason: &str, err_ext: &mut ErrorExtensionValues) {
  err_ext.set("code", code.as_u16());
  err_ext.set("reason", reason);
}
