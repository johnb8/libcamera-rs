use std::fmt;

use crate::bridge::{ffi, GetInner};

use crate::Result;

pub use ffi::DefaultPixelFormat;

/// Represents the configuration for a camera.
pub struct CameraConfig {
  inner: ffi::BindCameraConfiguration,
  streams: Vec<StreamConfig>,
}

impl CameraConfig {
  pub(crate) fn wrap_inner(mut inner: ffi::BindCameraConfiguration) -> Result<CameraConfig> {
    let streams = (0..unsafe { inner.get().size() })
      .map(|n| {
        Ok(StreamConfig::wrap_inner(unsafe {
          inner.get_mut().at(n.try_into()?)
        }?))
      })
      .collect::<Result<Vec<_>>>()?;
    Ok(CameraConfig { inner, streams })
  }
  pub(crate) fn get_inner(&mut self) -> &mut ffi::BindCameraConfiguration {
    &mut self.inner
  }
  pub fn streams(&self) -> &Vec<StreamConfig> {
    &self.streams
  }
  pub fn streams_mut(&mut self) -> &mut Vec<StreamConfig> {
    &mut self.streams
  }
}

impl fmt::Debug for CameraConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("CameraConfig")
      .field("streams", &self.streams)
      .finish_non_exhaustive()
  }
}

/// Represents the configuration for a stream in a camera.
pub struct StreamConfig {
  inner: ffi::BindStreamConfiguration,
}

impl StreamConfig {
  pub(crate) fn wrap_inner(inner: ffi::BindStreamConfiguration) -> Self {
    StreamConfig { inner }
  }
  pub(crate) fn get_inner(&self) -> &ffi::BindStreamConfiguration {
    &self.inner
  }
  /// Set the pixel format for this stream to a [DefaultPixelFormat]
  pub fn set_default_pixel_format(&mut self, fmt: DefaultPixelFormat) {
    unsafe {
      self
        .inner
        .get_mut()
        .set_pixel_format(ffi::get_default_pixel_format(fmt))
    };
  }
  pub fn set_size(&mut self, width: u32, height: u32) {
    unsafe { self.inner.get_mut().set_size(ffi::new_size(width, height)) };
  }
}

impl fmt::Debug for StreamConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StreamConfig")
      .field("description", &unsafe { self.inner.get().raw_to_string() })
      .finish_non_exhaustive()
  }
}
