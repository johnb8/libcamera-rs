use thiserror::Error;

mod bridge;
pub mod camera;
pub mod config;
pub mod controls;
pub mod image;
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
  #[error("No buffer ready for capture (all buffers in use, capture pictures slower!)")]
  NoBufferReady,
  #[error("Control value out of range!")]
  InvalidControlValue,
  #[error("Unknown ID in camera request")]
  UnknownRequestId,
  #[cfg(feature = "image")]
  #[error("Image error: {0}")]
  ImageError(#[from] ::image::ImageError),
  #[cfg(feature = "image")]
  #[error("Image error: Bad image format.")]
  BadImageFormat,
}

type Result<T> = std::result::Result<T, LibcameraError>;
