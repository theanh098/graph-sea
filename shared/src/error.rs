use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
  AuthenticationError,
  ExecutionError(String),
  DatabaseSeaError(String),
  DatabaseRecordNotFoundError {
    table: String,
    col: String,
    value: String,
  },
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use AppError::*;

    match self {
      AuthenticationError => write!(f, "UnAuthorized"),
      ExecutionError(reason) => write!(f, "ExecutionError reason: {}", reason),
      DatabaseSeaError(reason) => write!(f, "DatabaseSeaError reason: {}", reason),
      DatabaseRecordNotFoundError { col, table, value } => {
        write!(
          f,
          "DatabaseRecordNotFoundError reason: Not found record on table {} with {} is {}",
          table, col, value
        )
      }
    }
  }
}
