#![deny(clippy::all)]

use thiserror::Error;

mod bridge;
pub mod camera;
pub mod config;
pub mod prelude;
#[cfg(test)]
mod test;

#[derive(Debug, Error)]
pub enum LibcameraError {
  #[error("Inner C++ error: {0}")]
  InnerError(#[from] cxx::Exception),
  #[error("Int conversion error: {0}")]
  IntConversion(#[from] std::num::TryFromIntError),
  #[error("Configuration Invalid or Missing")]
  InvalidConfig,
}

type Result<T> = std::result::Result<T, LibcameraError>;
