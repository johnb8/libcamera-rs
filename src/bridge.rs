#![allow(dead_code)]

use std::pin::Pin;

#[cfg(test)]
mod test;

#[cxx::bridge]
pub mod ffi {
  /// Represents a "role" that a camera stream can be optimized for.
  #[namespace = "libcamera"]
  #[repr(i32)]
  #[derive(Debug)]
  enum StreamRole {
    /// ???
    Raw,
    /// Capturing still images
    StillCapture,
    /// Recording a video
    VideoRecording,
    /// Displaying a viewfinder
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

  /// Represents common pixel formats.
  #[repr(i32)]
  #[derive(Debug)]
  enum DefaultPixelFormat {
    /// 8bpp, single channel image
    R8,
    /// 24bpp, three channel image, order red, green, blue
    Rgb888,
    /// 16bpp, three channel image, order red, green, blue
    Rgb565,
    /// 24bpp, three channel image, order blue, green, red
    Bgr888,
    /// 16bpp, three channel image, YUV (4:2:2) encoding, order Y' U Y' V
    Yuyv,
    /// 16bpp, three channel image, YUV (4:2:2) encoding, order Y' V Y' U
    Yvyu,
    /// 16bpp*, chroma subsampling (U, V half width + height), three channel image, YUV (4:2:0) encoing, order Y', U, V
    Yuv420,
    /// 16bpp*, chroma subsampling (U, V half width), three channel image, YUV (4:2:2) encoing, order Y', U, V
    Yuv422,
    /// MJPEG (motion JPEG) encoding, effectively one JPEG image per frame
    Mjpeg,
  }

  #[repr(i32)]
  #[derive(Debug)]
  enum BindErrorCode {
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
    ENoExec = 8,
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

  struct BindCameraManager {
    inner: UniquePtr<CameraManager>,
  }
  struct BindCamera {
    inner: UniquePtr<Camera>,
  }
  struct BindCameraConfiguration {
    inner: UniquePtr<CameraConfiguration>,
  }
  struct BindPixelFormat {
    inner: UniquePtr<PixelFormat>,
  }
  struct BindSize {
    inner: UniquePtr<Size>,
  }
  struct BindStreamConfiguration {
    inner: UniquePtr<StreamConfiguration>,
  }
  struct BindStream {
    inner: UniquePtr<Stream>,
  }
  struct BindFrameBufferAllocator {
    inner: UniquePtr<FrameBufferAllocator>,
  }
  struct BindFrameBuffer {
    inner: UniquePtr<FrameBuffer>,
  }
  struct BindFrameBufferPlane {
    inner: UniquePtr<FrameBufferPlane>,
  }
  struct BindMemoryBuffer {
    inner: UniquePtr<MemoryBuffer>,
  }
  struct BindRequest {
    inner: UniquePtr<Request>,
  }
  struct BindControlId {
    inner: UniquePtr<ControlId>,
  }
  struct BindControlValue {
    inner: UniquePtr<ControlValue>,
  }

  #[repr(i32)]
  #[derive(Debug)]
  enum CameraMessageType {
    RequestComplete,
    BufferComplete,
  }

  #[derive(Debug)]
  struct CameraMessage {
    message_type: CameraMessageType,
    request_cookie: u64,
    buffer_cookie: u64,
  }

  #[repr(i32)]
  #[derive(Debug)]
  enum CameraControlType {
    None = 0,
    Bool = 1,
    Byte = 2,
    Integer32 = 3,
    Integer64 = 4,
    Float = 5,
    String = 6,
    Rectangle = 7,
    Size = 8,
  }

  struct ControlPair {
    id: BindControlId,
    min: BindControlValue,
    max: BindControlValue,
    value: BindControlValue,
  }

  extern "C++" {
    include!("libcamera-rs/libcamera-bridge/core.hpp");

    #[namespace = "libcamera"]
    type StreamRole;
    type CameraConfigurationStatus;

    type CameraManager;
    pub unsafe fn make_camera_manager() -> BindCameraManager;

    pub unsafe fn start(self: Pin<&mut CameraManager>) -> Result<()>;
    pub unsafe fn stop(self: Pin<&mut CameraManager>);
    pub unsafe fn get_camera_ids(self: &CameraManager) -> Vec<String>;
    pub unsafe fn get_camera_by_id(self: Pin<&mut CameraManager>, id: &str) -> Result<BindCamera>;

    type Camera;
    pub unsafe fn acquire(self: Pin<&mut Camera>) -> Result<()>;
    pub unsafe fn release(self: Pin<&mut Camera>) -> Result<()>;
    pub unsafe fn generate_configuration(
      self: Pin<&mut Camera>,
      roles: &[StreamRole],
    ) -> Result<BindCameraConfiguration>;
    pub unsafe fn configure(
      self: Pin<&mut Camera>,
      conf: Pin<&mut CameraConfiguration>,
    ) -> Result<()>;
    pub unsafe fn create_request(self: Pin<&mut Camera>, cookie: u64) -> Result<BindRequest>;
    pub unsafe fn queue_request(self: Pin<&mut Camera>, req: Pin<&mut Request>) -> Result<()>;
    pub unsafe fn start(self: Pin<&mut Camera>) -> Result<()>;
    pub unsafe fn stop(self: Pin<&mut Camera>) -> Result<()>;
    pub unsafe fn get_controls(self: &Camera) -> Vec<ControlPair>;
    pub unsafe fn poll_events(self: Pin<&mut Camera>) -> Vec<CameraMessage>;
    pub unsafe fn poll_events_with_cookie(
      self: Pin<&mut Camera>,
      request_cookie: u64,
    ) -> Vec<CameraMessage>;

    type CameraConfiguration;
    pub unsafe fn size(self: &CameraConfiguration) -> usize;
    pub unsafe fn at(
      self: Pin<&mut CameraConfiguration>,
      idx: u32,
    ) -> Result<BindStreamConfiguration>;
    pub unsafe fn validate(self: Pin<&mut CameraConfiguration>) -> CameraConfigurationStatus;

    type StreamConfiguration;
    pub unsafe fn stream(self: &StreamConfiguration) -> BindStream;
    pub unsafe fn set_pixel_format(
      self: Pin<&mut StreamConfiguration>,
      pixel_format: BindPixelFormat,
    );
    pub unsafe fn get_pixel_format(self: &StreamConfiguration) -> BindPixelFormat;
    pub unsafe fn set_size(self: Pin<&mut StreamConfiguration>, size: BindSize);
    pub unsafe fn get_size(self: &StreamConfiguration) -> BindSize;
    pub unsafe fn set_buffer_count(self: Pin<&mut StreamConfiguration>, buffer_count: usize);
    pub unsafe fn get_buffer_count(self: &StreamConfiguration) -> usize;
    pub unsafe fn raw_to_string(self: &StreamConfiguration) -> String;

    pub unsafe fn get_default_pixel_format(default_format: DefaultPixelFormat) -> BindPixelFormat;

    type PixelFormat;
    pub unsafe fn as_default_pixel_format(self: &PixelFormat) -> Result<DefaultPixelFormat>;
    pub unsafe fn raw_to_string(self: &PixelFormat) -> String;

    type Size;
    pub unsafe fn new_size(width: u32, height: u32) -> BindSize;
    pub unsafe fn set_width(self: Pin<&mut Size>, width: u32);
    pub unsafe fn get_width(self: &Size) -> u32;
    pub unsafe fn set_height(self: Pin<&mut Size>, height: u32);
    pub unsafe fn get_height(self: &Size) -> u32;
    pub unsafe fn raw_to_string(self: &Size) -> String;

    type Stream;

    type FrameBufferAllocator;
    pub unsafe fn make_frame_buffer_allocator(camera: Pin<&mut Camera>)
      -> BindFrameBufferAllocator;

    pub unsafe fn allocate(
      self: Pin<&mut FrameBufferAllocator>,
      stream: Pin<&mut Stream>,
    ) -> Result<usize>;
    pub unsafe fn free(
      self: Pin<&mut FrameBufferAllocator>,
      stream: Pin<&mut Stream>,
    ) -> Result<()>;
    pub unsafe fn buffers(
      self: &FrameBufferAllocator,
      stream: Pin<&mut Stream>,
    ) -> Vec<BindFrameBuffer>;

    type FrameBuffer;
    pub unsafe fn planes(self: &FrameBuffer) -> Vec<BindFrameBufferPlane>;
    pub unsafe fn set_cookie(self: Pin<&mut FrameBuffer>, cookie: u64);
    pub unsafe fn get_cookie(self: &FrameBuffer) -> u64;

    type FrameBufferPlane;
    pub unsafe fn get_fd(self: &FrameBufferPlane) -> i32;
    pub unsafe fn get_offset(self: &FrameBufferPlane) -> usize;
    pub unsafe fn get_length(self: &FrameBufferPlane) -> usize;

    /// File descriptor functions
    pub unsafe fn fd_len(fd: i32) -> Result<usize>;
    pub unsafe fn mmap_plane(fd: i32, length: usize) -> Result<BindMemoryBuffer>;

    type MemoryBuffer;
    pub unsafe fn sub_buffer(
      self: Pin<&mut MemoryBuffer>,
      offset: usize,
      length: usize,
    ) -> Result<BindMemoryBuffer>;
    pub unsafe fn read_to_vec(self: &MemoryBuffer) -> Vec<u8>;

    type Request;
    pub unsafe fn add_buffer(
      self: Pin<&mut Request>,
      stream: &Stream,
      buffer: Pin<&mut FrameBuffer>,
    ) -> Result<()>;
    pub unsafe fn get_control(self: &Request, id: u32) -> Result<BindControlValue>;
    pub unsafe fn set_control(self: Pin<&mut Request>, id: u32, value: &ControlValue);
    pub unsafe fn raw_to_string(self: &Request) -> String;

    type ControlId;

    pub unsafe fn get_name(self: &ControlId) -> String;
    pub unsafe fn get_id(self: &ControlId) -> u32;
    pub unsafe fn get_type(self: &ControlId) -> CameraControlType;

    type ControlValue;

    pub unsafe fn new_control_value_bool(value: bool) -> BindControlValue;
    pub unsafe fn new_control_value_u8(value: u8) -> BindControlValue;
    pub unsafe fn new_control_value_i32(value: i32) -> BindControlValue;
    pub unsafe fn new_control_value_i64(value: i64) -> BindControlValue;
    pub unsafe fn new_control_value_f32(value: f32) -> BindControlValue;
    pub unsafe fn new_control_value_string(value: String) -> BindControlValue;

    pub unsafe fn get_bool(self: &ControlValue) -> Result<bool>;
    pub unsafe fn get_u8(self: &ControlValue) -> Result<u8>;
    pub unsafe fn get_i32(self: &ControlValue) -> Result<i32>;
    pub unsafe fn get_i64(self: &ControlValue) -> Result<i64>;
    pub unsafe fn get_f32(self: &ControlValue) -> Result<f32>;

    pub unsafe fn raw_to_string(self: &ControlValue) -> String;
  }
}

/// # Safety
/// The inner pointer to the libcamera object must be valid.
pub unsafe trait GetInner {
  type Inner;
  unsafe fn get(&self) -> &Self::Inner;
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner>;
}

unsafe impl GetInner for ffi::BindCameraManager {
  type Inner = ffi::CameraManager;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindCamera {
  type Inner = ffi::Camera;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindCameraConfiguration {
  type Inner = ffi::CameraConfiguration;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindStreamConfiguration {
  type Inner = ffi::StreamConfiguration;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindPixelFormat {
  type Inner = ffi::PixelFormat;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindSize {
  type Inner = ffi::Size;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindStream {
  type Inner = ffi::Stream;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindFrameBufferAllocator {
  type Inner = ffi::FrameBufferAllocator;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindFrameBuffer {
  type Inner = ffi::FrameBuffer;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindFrameBufferPlane {
  type Inner = ffi::FrameBufferPlane;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindMemoryBuffer {
  type Inner = ffi::MemoryBuffer;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindRequest {
  type Inner = ffi::Request;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindControlId {
  type Inner = ffi::ControlId;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}

unsafe impl GetInner for ffi::BindControlValue {
  type Inner = ffi::ControlValue;
  unsafe fn get(&self) -> &Self::Inner {
    &self.inner
  }
  unsafe fn get_mut(&mut self) -> Pin<&mut Self::Inner> {
    self.inner.pin_mut()
  }
}
