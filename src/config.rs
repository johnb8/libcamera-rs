use std::fmt;

use crate::bridge::{ffi, GetInner};

use crate::Result;

pub use ffi::DefaultPixelFormat as PixelFormat;

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
  /// Set the pixel format for this stream to a [PixelFormat]
  pub fn set_pixel_format(&mut self, fmt: PixelFormat) {
    unsafe {
      self
        .inner
        .get_mut()
        .set_pixel_format(ffi::get_default_pixel_format(fmt))
    };
  }
  /// Retrieve the current pixel format
  ///
  /// # Returns
  /// Returns None if the pixel format is not a known pixel format.
  pub fn get_pixel_format(&self) -> Option<PixelFormat> {
    let pixel_format = unsafe { self.inner.get().get_pixel_format() };
    unsafe { pixel_format.get().as_default_pixel_format() }.ok()
  }
  /// Set the target image size for this stream.
  pub fn set_size(&mut self, width: u32, height: u32) {
    unsafe { self.inner.get_mut().set_size(ffi::new_size(width, height)) };
  }
  /// Get the target image size for this stream.
  pub fn get_size(&self) -> (u32, u32) {
    let size = unsafe { self.inner.get().get_size() };
    (unsafe { size.get().get_width() }, unsafe {
      size.get().get_height()
    })
  }
  pub fn description(&self) -> String {
    unsafe { self.inner.get().raw_to_string() }
  }
}

impl fmt::Debug for StreamConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StreamConfig")
      .field("size", &self.get_size())
      .field("pixel_format", &self.get_pixel_format())
      .finish_non_exhaustive()
  }
}
