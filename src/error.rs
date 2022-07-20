use thiserror::Error;

/// An error.
#[derive(Debug, Error)]
pub enum LibcameraError {
  /// A C++ exception
  #[error("Inner C++ error: {0}")]
  InnerError(#[from] cxx::Exception),
  /// An error converting an integer
  #[error("Int conversion error: {0}")]
  IntConversion(#[from] std::num::TryFromIntError),
  /// An error when validating configuration
  #[error("Configuration Invalid or Missing")]
  InvalidConfig,
  /// An error produced when it an image capture is requested but there are no buffers available.
  #[error("No buffer ready for capture (all buffers in use, capture pictures slower!)")]
  NoBufferReady,
  /// An error emitted when a control value is attempted to be set to a value outside of the acceptable range.
  #[error("Control value ({0:?}) out of range!")]
  InvalidControlValue(Box<dyn std::fmt::Debug + Send + Sync>),
  /// An error reading a control value
  #[error("Error converting control value!")]
  ControlValueError,
  /// An error en/decoding an image.
  #[cfg(feature = "image")]
  #[error("Image error: {0}")]
  ImageError(#[from] ::image::ImageError),
  /// An error produced when the decoded JPEG image is not in RGB888 format.
  #[cfg(feature = "image")]
  #[error("Image error: Bad image format.")]
  BadImageFormat,
}

/// Internal result type for convenience.
pub type Result<T> = std::result::Result<T, LibcameraError>;
