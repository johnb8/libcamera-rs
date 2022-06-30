use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use crate::bridge::{ffi, GetInner};
use crate::config::CameraConfig;
use crate::{LibcameraError, Result};

pub use ffi::StreamRole;

/// Manages cameras
pub struct CameraManager {
  inner: ffi::BindCameraManager,
}

impl fmt::Debug for CameraManager {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("CameraManager").finish_non_exhaustive()
  }
}

impl CameraManager {
  /// Constructs a new camera manager
  pub fn new() -> Result<CameraManager> {
    let mut cm = ffi::make_camera_manager();
    // The primary safety concern for the CM is that it must be started once before calling all functions.
    unsafe { cm.get_mut().start() }?;
    Ok(CameraManager { inner: cm })
  }
  /// Get a list of all attached cameras
  pub fn get_camera_names(&self) -> Vec<String> {
    unsafe { self.inner.get().get_camera_ids() }
  }
  /// Get a camera with a given name
  pub fn get_camera_by_name(&mut self, name: &str) -> Result<Camera<'_>> {
    let mut cam = unsafe { self.inner.get_mut().get_camera_by_id(name) }?;
    unsafe { cam.get_mut().acquire() }?;
    let allocator = unsafe { ffi::make_frame_buffer_allocator(cam.get_mut()) };
    Ok(Camera {
      _camera_manager: PhantomData,
      name: name.to_string(),
      config: None,
      inner: cam,
      allocator,
      streams: Vec::new(),
      started: false,
    })
  }
}

impl Drop for CameraManager {
  fn drop(&mut self) {
    unsafe { self.inner.get_mut().stop() };
  }
}

struct CameraStream {
  next_buffer: usize,
  buffers: Vec<(ffi::BindRequest, Vec<ffi::BindMemoryBuffer>)>,
}

/// Represents a camera
pub struct Camera<'a> {
  _camera_manager: PhantomData<&'a CameraManager>,
  name: String,
  config: Option<CameraConfig>,
  inner: ffi::BindCamera,
  allocator: ffi::BindFrameBufferAllocator,
  streams: Vec<CameraStream>,
  started: bool,
}

impl fmt::Debug for Camera<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Camera")
      .field("name", &self.name)
      .finish_non_exhaustive()
  }
}

impl Camera<'_> {
  /// Generate a configuration for this camera using the given set of stream roles to generate an corresponding set of streams.
  pub fn generate_config(&mut self, caps: &[StreamRole]) -> Result<&mut CameraConfig> {
    let config = unsafe { self.inner.get_mut().generate_configuration(caps) }?;
    self.config = Some(CameraConfig::wrap_inner(config)?);
    self.config.as_mut().ok_or(LibcameraError::InvalidConfig)
  }
  /// Validate and apply the configuration previously generated by this camera.
  pub fn apply_config(&mut self) -> Result<ConfigStatus> {
    if let Some(config) = &mut self.config {
      let config_status = unsafe { config.get_inner().get_mut().validate() };
      let (set, result) = match config_status {
        ffi::CameraConfigurationStatus::Valid => (true, Ok(ConfigStatus::Unchanged)),
        ffi::CameraConfigurationStatus::Adjusted => (true, Ok(ConfigStatus::Changed)),
        _ => (false, Err(LibcameraError::InvalidConfig)),
      };
      if set {
        unsafe { self.inner.get_mut().configure(config.get_inner().get_mut()) }?;
      }
      result
    } else {
      Err(LibcameraError::InvalidConfig)
    }
  }
  /// Borrow this camera's config.
  pub fn get_config(&self) -> Option<&CameraConfig> {
    self.config.as_ref()
  }
  /// Start the camera so that it's ready to capture images.
  ///
  /// This should only be called once, future calls will do nothing and the camera's streams cannot be configured while it is started.
  /// # Returns
  /// On success returns whether the stream was newly started (i.e. false means the stream was already running).
  /// This will fail if the camera has not been properly configured, or if libcamera decides to not work.
  /// # Panics
  /// This will panic if the buffer sizes produced by libcamera extend past the end of the actual camera memory buffer.
  pub fn start_stream(&mut self) -> Result<bool> {
    if self.started {
      // Do nothing if the camera was already started.
      return Ok(false);
    }
    self.started = true;
    // Ok so:
    // The camera contains streams, each stream has multiple buffers and each buffer has multiple planes.
    // Each request to the camera operates on one buffer on one stream and fills the buffer with data from that stream.
    // To start the camera we must allocate the buffers for all the streams and save them somewhere for future reading.
    // We also must create a request for each buffer that we can re-use later every time we need an image.
    // Technically requests can have multiple buffers, but I don't think know why this would be the case and I don't think it's necessary.

    // For each stream...
    for stream_config in self
      .config
      .as_ref()
      .ok_or(LibcameraError::InvalidConfig)?
      .streams()
    {
      let mut stream = unsafe { stream_config.get_inner().get().stream() };
      // Allocate buffers
      let _buffer_count = unsafe { self.allocator.get_mut().allocate(stream.get_mut()) };
      let mut camera_stream = CameraStream {
        next_buffer: 0,
        buffers: Vec::new(),
      };
      // Create requests and map memory
      for mut buffer in unsafe { self.allocator.get().buffers(stream.get_mut()) } {
        let mut request = unsafe { self.inner.get_mut().create_request() }?;
        let mut planes = Vec::new();
        let mut mapped_buffers: HashMap<i32, (Option<ffi::BindMemoryBuffer>, usize, usize)> =
          HashMap::new();
        for plane in unsafe { buffer.get().planes() } {
          let fd = unsafe { plane.get().get_fd() };
          let mapped_buffer = mapped_buffers
            .entry(fd)
            .or_insert((None, 0, unsafe { ffi::fd_len(fd) }?));
          let length = mapped_buffer.2;
          let plane_offset = unsafe { plane.get().get_offset() };
          let plane_length = unsafe { plane.get().get_length() };
          if plane_offset + plane_length > length {
            panic!(
							"Plane is out of buffer: buffer length = {length}, plane offset = {}, plane length = {}",
							unsafe { plane.get().get_offset() },
							unsafe { plane.get().get_length() },
						);
          }
          mapped_buffer.1 = mapped_buffer.1.max(plane_offset + plane_length);
        }
        for plane in unsafe { buffer.get().planes() } {
          let fd = unsafe { plane.get().get_fd() };
          let mapped_buffer = mapped_buffers.get_mut(&fd).unwrap();
          if mapped_buffer.0.is_none() {
            mapped_buffer.0 = Some(unsafe { ffi::mmap_plane(fd, mapped_buffer.1) }?);
          }
          planes.push(unsafe {
            mapped_buffer
              .0
              .as_mut()
              .unwrap()
              .get_mut()
              .sub_buffer(plane.get().get_offset(), plane.get().get_length())
          }?);
        }

        unsafe { request.get_mut().add_buffer(stream.get(), buffer.get_mut()) }?;
        camera_stream.buffers.push((request, planes));
      }
      self.streams.push(camera_stream);
    }
    unsafe { self.inner.get_mut().start() }?;
    Ok(true)
  }
  /// Start the process to capture an image from the camera.
  pub fn capture_next_picture(&mut self, stream: usize) -> Result<()> {
    let mut stream = &mut self.streams[stream];
    let mut buffer = &mut stream.buffers[stream.next_buffer];
    unsafe { self.inner.get_mut().queue_request(buffer.0.get_mut()) }?;
    stream.next_buffer += 1;
    Ok(())
  }
}

impl Drop for Camera<'_> {
  fn drop(&mut self) {
    self.streams = Vec::new();
    unsafe { self.inner.get_mut().stop() }.unwrap();
    unsafe { self.inner.get_mut().release() }.unwrap();
  }
}

/// Represents the result of applying a configuration to a camera.
#[derive(Debug)]
pub enum ConfigStatus {
  /// The configuration was applied to the camera unchanged
  Unchanged,
  /// The configuration was applied to the camera, but some values have been adjusted by the driver to a supported configuration for this camera
  Changed,
}
