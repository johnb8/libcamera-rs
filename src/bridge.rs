use cxx::SharedPtr;
use std::pin::Pin;

#[cfg(test)]
mod test;

#[cxx::bridge]
pub mod ffi {
  struct BridgeCamera {
    inner: SharedPtr<Camera>,
  }

  #[namespace = "libcamera"]
  #[repr(i32)]
  #[derive(Debug)]
  enum StreamRole {
    Raw,
    StillCapture,
    VideoRecording,
    Viewfinder,
  }

  #[namespace = "libcamera"]
  #[repr(i32)]
  #[derive(Debug)]
  enum CameraConfigurationStatus {
    Valid,
    Adjusted,
    Invalid,
  }

  #[repr(i32)]
  #[derive(Debug)]
  enum DefaultPixelFormat {
    Rgb888,
    Bgr888,
    Yuv420,
    Mjpeg,
  }

  unsafe extern "C++" {
    include!("libcamera/stream.h");
    include!("libcamera/camera.h");
    include!("libcamera-rs/libcamera-bridge/core.hpp");

    // Camera Manager
    type CameraManager;

    pub fn make_camera_manager() -> UniquePtr<CameraManager>;
    pub fn start(self: Pin<&mut CameraManager>) -> Result<()>;
    pub fn stop(self: Pin<&mut CameraManager>);
    pub fn version(self: Pin<&mut CameraManager>) -> String;
    pub fn cameras(self: &CameraManager) -> Vec<BridgeCamera>;
    pub fn get(self: Pin<&mut CameraManager>, id: &CxxString) -> SharedPtr<Camera>;

    // Camera
    #[namespace = "libcamera"]
    type Camera;

    pub fn get_mut_camera(cam: &mut SharedPtr<Camera>) -> Pin<&mut Camera>;

    pub fn id(self: &Camera) -> &CxxString;
    pub fn acquire(self: Pin<&mut Camera>) -> i32;
    pub fn release(self: Pin<&mut Camera>) -> i32;
    pub fn stop(self: Pin<&mut Camera>) -> i32;

    pub fn generate_camera_configuration(
      cam: Pin<&mut Camera>,
      roles: &Vec<StreamRole>,
    ) -> UniquePtr<CameraConfiguration>;

    pub fn configure_camera(cam: Pin<&mut Camera>, config: Pin<&mut CameraConfiguration>);

    // Camera Configuration
    #[namespace = "libcamera"]
    type CameraConfiguration;

    pub fn at(self: Pin<&mut CameraConfiguration>, index: u32) -> Pin<&mut StreamConfiguration>;
    pub fn validate(self: Pin<&mut CameraConfiguration>) -> CameraConfigurationStatus;

    type CameraConfigurationStatus;

    #[namespace = "libcamera"]
    type StreamConfiguration;

    pub fn set_stream_pixel_format(
      stream: Pin<&mut StreamConfiguration>,
      format: Pin<&PixelFormat>,
    );
    pub fn set_stream_size(stream: Pin<&mut StreamConfiguration>, width: u32, height: u32);
    pub fn set_stream_buffer_count(stream: Pin<&mut StreamConfiguration>, buffer_count: u32);

    #[namespace = "libcamera"]
    type StreamRole;

    #[namespace = "libcamera"]
    type PixelFormat;

    pub fn get_default_pixel_format(format: DefaultPixelFormat) -> Pin<&'static PixelFormat>;
  }
}

pub trait MutFromSharedPtr {
  type Target;

  fn pin_mut(&mut self) -> Pin<&mut Self::Target>;
}

impl MutFromSharedPtr for SharedPtr<ffi::Camera> {
  type Target = ffi::Camera;

  fn pin_mut(&mut self) -> Pin<&mut Self::Target> {
    ffi::get_mut_camera(self)
  }
}
