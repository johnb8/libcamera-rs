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

  #[repr(i32)]
  #[derive(Debug)]
  enum CameraError {
    /// Operation not permitted
    EPerm = 1,
    /// No such file or directory
    ENoEnt = 2,
    /// No such process
    ESrch = 3,
    /// Interrupted system call
    EIntr = 4,
    /// I/O error
    EIo = 5,
    /// No such device or address
    ENxIo = 6,
    /// Argument list too long
    E2Big = 7,
    /// EXec format error
    ENoexec = 8,
    /// Bad file number
    EBadF = 9,
    /// No child processes
    EChild = 10,
    /// Try again
    EAgain = 11,
    /// Out of memory
    ENoMem = 12,
    /// Permission denied
    EAcces = 13,
    /// Bad address
    EFault = 14,
    /// Block device required
    ENotBlk = 15,
    /// Device or resource busy
    EBusy = 16,
    /// File exists
    EExist = 17,
    /// Cross-device link
    EXDev = 18,
    /// No such device
    ENoDev = 19,
    /// Not a directory
    ENotDir = 20,
    /// Is a directory
    EIsDir = 21,
    /// Invalid argument
    EInval = 22,
    /// File table overflow
    ENFile = 23,
    /// Too many open files
    EMFile = 24,
    /// Not a typewriter
    ENotTy = 25,
    /// Text file busy
    ETxtBsy = 26,
    /// File too large
    EFBig = 27,
    /// No space left on device
    ENoSpc = 28,
    /// Illegal seek
    ESPipe = 29,
    /// Read-only file system
    ERoFs = 30,
    /// Too many links
    EMLink = 31,
    /// Broken pipe
    EPipe = 32,
    /// Math argument out of domain of func
    EDom = 33,
    /// Math result not representable
    ERange = 34,
  }

  unsafe extern "C++" {
    include!("libcamera/stream.h");
    include!("libcamera/camera.h");
    include!("libcamera/framebuffer_allocator.h");
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

    pub fn connect_camera_buffer_completed(
      cam: Pin<&mut Camera>,
      callback: fn(request: &Request, frame_buffer: &FrameBuffer),
    );
    pub fn connect_camera_request_completed(cam: Pin<&mut Camera>, callback: fn(request: &Request));
    pub fn connect_camera_disconnected(cam: Pin<&mut Camera>, callback: fn());

    // Frame Buffers
    #[namespace = "libcamera"]
    type FrameBufferAllocator;

    pub fn make_frame_buffer_allocator(cam: &SharedPtr<Camera>) -> UniquePtr<FrameBufferAllocator>;

    pub fn allocate_frame_buffer_stream(
      alloc: Pin<&mut FrameBufferAllocator>,
      stream: Pin<&mut Stream>,
    ) -> Result<u32>;

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

    pub fn get_stream_from_configuration(conf: Pin<&mut StreamConfiguration>) -> Pin<&mut Stream>;

    #[namespace = "libcamera"]
    type Stream;

    // Misc. Types

    #[namespace = "libcamera"]
    type StreamRole;

    #[namespace = "libcamera"]
    type PixelFormat;

    pub fn get_default_pixel_format(format: DefaultPixelFormat) -> Pin<&'static PixelFormat>;

    #[namespace = "libcamera"]
    type Request;

    #[namespace = "libcamera"]
    type FrameBuffer;
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
