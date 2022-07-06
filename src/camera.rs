use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::RangeInclusive;

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
    let mut cm = unsafe { ffi::make_camera_manager() };
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
    let controls = CameraControls::from_libcamera(unsafe { cam.get().get_controls() });
    Ok(Camera {
      _camera_manager: PhantomData,
      name: name.to_string(),
      config: None,
      inner: cam,
      allocator,
      streams: Vec::new(),
      started: false,
      controls,
    })
  }
}

impl Drop for CameraManager {
  fn drop(&mut self) {
    unsafe { self.inner.get_mut().stop() };
  }
}

struct CameraBuffer {
  buffer_id: u32,
  buffer: ffi::BindFrameBuffer,
  request: Option<ffi::BindRequest>,
  planes: Vec<ffi::BindMemoryBuffer>,
}

impl fmt::Debug for CameraBuffer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("CameraBuffer")
      .field("buffer_id", &self.buffer_id)
      .field("plane_count", &self.planes.len())
      .finish_non_exhaustive()
  }
}

struct CameraStream {
  stream_id: u32,
  stream: ffi::BindStream,
  next_buffer: usize,
  buffers: Vec<CameraBuffer>,
}

impl fmt::Debug for CameraStream {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("CameraStream")
      .field("stream_id", &self.stream_id)
      .field("next_buffer", &self.next_buffer)
      .field("buffers", &self.buffers)
      .finish_non_exhaustive()
  }
}

/// Contains a value with an acceptable minimum and maximum and a default.
#[derive(Debug)]
pub struct MinMaxValue<T: PartialOrd + Copy + Debug> {
  range: RangeInclusive<T>,
  default: T,
  value: T,
}

impl<T: PartialOrd + Copy + Debug> MinMaxValue<T> {
  /// Creates a new MinMaxValue out of a given min, max, and default
  ///
  /// # Returns
  /// Returns None if default is not within min and max.
  pub fn new(min: T, max: T, default: T) -> Result<MinMaxValue<T>> {
    let range = min..=max;
    if range.contains(&default) {
      Ok(MinMaxValue {
        range: min..=max,
        default,
        value: default,
      })
    } else {
      Err(LibcameraError::InvalidControlValue)
    }
  }
  /// Retrieve the default value
  pub fn get_default(&self) -> T {
    self.default
  }
  /// Retrieve the minimum value
  pub fn min(&self) -> T {
    *self.range.start()
  }
  /// Retrieve the maximum value
  pub fn max(&self) -> T {
    *self.range.end()
  }
  /// Gets the stored value
  /// It is gurenteed to lie within MinMaxValue::min() and MinMaxValue::max().
  pub fn get_value(&self) -> T {
    self.value
  }
  /// Verifies that value lies within the acceptable range for this value
  ///
  /// # Returns
  /// `true` if the value lies within the acceptable range for this value and was stored, `false` otherwise.
  pub fn set_value(&mut self, value: T) -> bool {
    if self.range.contains(&value) {
      self.value = value;
      true
    } else {
      false
    }
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<bool> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<bool>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_bool() }?,
      unsafe { pair.max.get().get_bool() }?,
      unsafe { pair.value.get().get_bool() }?,
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<u8> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<u8>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_u8() }?,
      unsafe { pair.max.get().get_u8() }?,
      unsafe { pair.value.get().get_u8() }?,
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<i32> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<i32>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_i32() }?,
      unsafe { pair.max.get().get_i32() }?,
      unsafe { pair.value.get().get_i32() }?,
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<i64> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<i64>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_i64() }?,
      unsafe { pair.max.get().get_i64() }?,
      unsafe { pair.value.get().get_i64() }?,
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<f32> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<f32>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_f32() }?,
      unsafe { pair.max.get().get_f32() }?,
      unsafe { pair.value.get().get_f32() }?,
    )
  }
}

/// Represents a camera control value with an unknown type
///
/// Most of the time you probably want to use `CameraControls` instead.
#[non_exhaustive]
#[derive(Debug)]
pub enum CameraControlValue {
  None,
  Bool(MinMaxValue<bool>),
  Byte(MinMaxValue<u8>),
  Integer32(MinMaxValue<i32>),
  Integer64(MinMaxValue<i64>),
  Float(MinMaxValue<f32>),
  // String(MinMaxValue<String>),
  // Rectangle(MinMaxValue<Rectangle>),
  // Size(MinMaxValue<Size>),
}

/// Stores camera controls.
///
/// Common controls are fields on this struct
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct CameraControls {
  pub ae_enable: Option<MinMaxValue<bool>>,
  pub ae_metering_mode: Option<MinMaxValue<i32>>,
  pub ae_constraint_mode: Option<MinMaxValue<i32>>,
  pub ae_exposure_mode: Option<MinMaxValue<i32>>,
  pub exposure_value: Option<MinMaxValue<f32>>,
  pub exposure_time: Option<MinMaxValue<i32>>,
  pub analogue_gain: Option<MinMaxValue<f32>>,
  pub brightness: Option<MinMaxValue<f32>>,
  pub contract: Option<MinMaxValue<f32>>,
  pub awb_enable: Option<MinMaxValue<bool>>,
  pub awb_mode: Option<MinMaxValue<i32>>,
  pub colour_gains: Option<MinMaxValue<f32>>,
  pub saturation: Option<MinMaxValue<f32>>,
  pub sharpness: Option<MinMaxValue<f32>>,
  pub colour_correction_matrix: Option<MinMaxValue<f32>>,
  // pub scaler_crop: Option<MinMaxValue<Rectangle>>, // Rectangle TODO
  pub frame_duration_limits: Option<MinMaxValue<i64>>,
  pub noise_reduction_mode: Option<MinMaxValue<i32>>,
  pub others: HashMap<u32, (String, CameraControlValue)>,
}

impl CameraControls {
  fn from_libcamera(control_list: Vec<ffi::ControlPair>) -> Self {
    let mut controls = CameraControls::default();
    for control in control_list {
      let name = unsafe { control.id.get().get_name() };
      let did_name_match = match name.as_ref() {
        "AeEnable" => (&control)
          .try_into()
          .map(|control| controls.ae_enable = Some(control))
          .is_ok(),
        "AeMeteringMode" => (&control)
          .try_into()
          .map(|control| controls.ae_metering_mode = Some(control))
          .is_ok(),
        "AeConstraintMode" => (&control)
          .try_into()
          .map(|control| controls.ae_constraint_mode = Some(control))
          .is_ok(),
        "AeExposureMode" => (&control)
          .try_into()
          .map(|control| controls.ae_exposure_mode = Some(control))
          .is_ok(),
        "ExposureValue" => (&control)
          .try_into()
          .map(|control| controls.exposure_value = Some(control))
          .is_ok(),
        "ExposureTime" => (&control)
          .try_into()
          .map(|control| controls.exposure_time = Some(control))
          .is_ok(),
        "AnalogueGain" => (&control)
          .try_into()
          .map(|control| controls.analogue_gain = Some(control))
          .is_ok(),
        "Brightness" => (&control)
          .try_into()
          .map(|control| controls.brightness = Some(control))
          .is_ok(),
        "Contract" => (&control)
          .try_into()
          .map(|control| controls.contract = Some(control))
          .is_ok(),
        "AwbEnable" => (&control)
          .try_into()
          .map(|control| controls.awb_enable = Some(control))
          .is_ok(),
        "AwbMode" => (&control)
          .try_into()
          .map(|control| controls.awb_mode = Some(control))
          .is_ok(),
        "ColourGains" => (&control)
          .try_into()
          .map(|control| controls.colour_gains = Some(control))
          .is_ok(),
        "Saturation" => (&control)
          .try_into()
          .map(|control| controls.saturation = Some(control))
          .is_ok(),
        "Sharpness" => (&control)
          .try_into()
          .map(|control| controls.sharpness = Some(control))
          .is_ok(),
        "ColourCorrectionMatrix" => (&control)
          .try_into()
          .map(|control| controls.colour_correction_matrix = Some(control))
          .is_ok(),
        // "ScalerCrop" => (&control).try_into().map(|control| controls.scaler_crop = Some(control)).is_ok(),
        "FrameDurationLimits" => (&control)
          .try_into()
          .map(|control| controls.frame_duration_limits = Some(control))
          .is_ok(),
        "NoiseReductionMode" => (&control)
          .try_into()
          .map(|control| controls.noise_reduction_mode = Some(control))
          .is_ok(),
        _ => false,
      };
      if !did_name_match {
        if let Some(control_value) = match unsafe { control.id.get().get_type() } {
          ffi::CameraControlType::None => Some(CameraControlValue::None),
          ffi::CameraControlType::Bool => (&control).try_into().ok().map(CameraControlValue::Bool),
          ffi::CameraControlType::Byte => (&control).try_into().ok().map(CameraControlValue::Byte),
          ffi::CameraControlType::Integer32 => (&control)
            .try_into()
            .ok()
            .map(CameraControlValue::Integer32),
          ffi::CameraControlType::Integer64 => (&control)
            .try_into()
            .ok()
            .map(CameraControlValue::Integer64),
          ffi::CameraControlType::Float => {
            (&control).try_into().ok().map(CameraControlValue::Float)
          }
          _ => None,
          // ffi::CameraControlType::String => (&control).try_into().ok().map(|control| CameraControlValue::String(control)),
          // ffi::CameraControlType::Rectangle => (&control).try_into().ok().map(|control| CameraControlValue::Rectangle(control)),
          // ffi::CameraControlType::Size => (&control).try_into().ok().map(|control| CameraControlValue::Size(control)),
        } {
          controls
            .others
            .insert(unsafe { control.id.get().get_id() }, (name, control_value));
        } else {
          eprintln!("Camera control with conflicting types: {name}");
        }
      }
    }
    controls
  }
  fn get_libcamera(&self) -> Vec<(u32, ffi::BindControlValue)> {
    let mut controls = Vec::new();
    if let Some(ae_enable) = &self.ae_enable {
      controls.push((1, unsafe {
        ffi::new_control_value_bool(ae_enable.get_value())
      }));
    }
    if let Some(ae_metering_mode) = &self.ae_metering_mode {
      controls.push((3, unsafe {
        ffi::new_control_value_i32(ae_metering_mode.get_value())
      }));
    }
    if let Some(ae_constraint_mode) = &self.ae_constraint_mode {
      controls.push((4, unsafe {
        ffi::new_control_value_i32(ae_constraint_mode.get_value())
      }));
    }
    if let Some(ae_exposure_mode) = &self.ae_exposure_mode {
      controls.push((5, unsafe {
        ffi::new_control_value_i32(ae_exposure_mode.get_value())
      }));
    }
    if let Some(exposure_value) = &self.exposure_value {
      controls.push((6, unsafe {
        ffi::new_control_value_f32(exposure_value.get_value())
      }));
    }
    if let Some(exposure_time) = &self.exposure_time {
      controls.push((7, unsafe {
        ffi::new_control_value_i32(exposure_time.get_value())
      }));
    }
    if let Some(analogue_gain) = &self.analogue_gain {
      controls.push((8, unsafe {
        ffi::new_control_value_f32(analogue_gain.get_value())
      }));
    }
    if let Some(brightness) = &self.brightness {
      controls.push((9, unsafe {
        ffi::new_control_value_f32(brightness.get_value())
      }));
    }
    if let Some(contract) = &self.contract {
      controls.push((10, unsafe {
        ffi::new_control_value_f32(contract.get_value())
      }));
    }
    if let Some(awb_enable) = &self.awb_enable {
      controls.push((12, unsafe {
        ffi::new_control_value_bool(awb_enable.get_value())
      }));
    }
    if let Some(awb_mode) = &self.awb_mode {
      controls.push((13, unsafe {
        ffi::new_control_value_i32(awb_mode.get_value())
      }));
    }
    if let Some(colour_gains) = &self.colour_gains {
      controls.push((15, unsafe {
        ffi::new_control_value_f32(colour_gains.get_value())
      }));
    }
    if let Some(saturation) = &self.saturation {
      controls.push((17, unsafe {
        ffi::new_control_value_f32(saturation.get_value())
      }));
    }
    if let Some(sharpness) = &self.sharpness {
      controls.push((19, unsafe {
        ffi::new_control_value_f32(sharpness.get_value())
      }));
    }
    if let Some(colour_correction_matrix) = &self.colour_correction_matrix {
      controls.push((21, unsafe {
        ffi::new_control_value_f32(colour_correction_matrix.get_value())
      }));
    }
    // if let Some(scaler_crop) = &self.scaler_crop {
    //   controls.push((22, unsafe { ffi::new_control_value_rectangle(scaler_crop.get_value()) }));
    // }
    if let Some(frame_duration_limits) = &self.frame_duration_limits {
      controls.push((25, unsafe {
        ffi::new_control_value_i64(frame_duration_limits.get_value())
      }));
    }
    if let Some(noise_reduction_mode) = &self.noise_reduction_mode {
      controls.push((39, unsafe {
        ffi::new_control_value_i32(noise_reduction_mode.get_value())
      }));
    }
    for (id, (_name, value)) in &self.others {
      if let Some(value) = match value {
        CameraControlValue::None => None,
        CameraControlValue::Bool(value) => {
          Some(unsafe { ffi::new_control_value_bool(value.get_value()) })
        }
        CameraControlValue::Byte(value) => {
          Some(unsafe { ffi::new_control_value_u8(value.get_value()) })
        }
        CameraControlValue::Integer32(value) => {
          Some(unsafe { ffi::new_control_value_i32(value.get_value()) })
        }
        CameraControlValue::Integer64(value) => {
          Some(unsafe { ffi::new_control_value_i64(value.get_value()) })
        }
        CameraControlValue::Float(value) => {
          Some(unsafe { ffi::new_control_value_f32(value.get_value()) })
        }
        // CameraControlValue::String(value) => Some(unsafe { ffi::new_control_value_string(value.get_value()) }),
        // CameraControlValue::Rectangle(value) => Some(unsafe { ffi::new_control_value_rectangle(value.get_value()) }),
        // CameraControlValue::Size(value) => Some(unsafe { ffi::new_control_value_size(value.get_value()) }),
      } {
        controls.push((*id, value));
      }
    }
    controls
  }
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
  controls: CameraControls,
}

impl fmt::Debug for Camera<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Camera")
      .field("name", &self.name)
      .field("config", &self.config)
      .field("streams", &self.streams)
      .field("started", &self.started)
      .field("controls", &self.controls)
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
  /// Borrow the camera's controls
  pub fn get_controls(&self) -> &CameraControls {
    &self.controls
  }
  /// Borrow the camera's controls mutably
  pub fn get_controls_mut(&mut self) -> &mut CameraControls {
    &mut self.controls
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
      let stream_id = self.streams.len() as u32;
      let mut camera_stream = CameraStream {
        stream_id,
        stream,
        next_buffer: 0,
        buffers: Vec::new(),
      };
      // Map memory for buffers
      for mut buffer in unsafe { self.allocator.get().buffers(camera_stream.stream.get_mut()) } {
        let buffer_id = camera_stream.buffers.len() as u32;
        unsafe { buffer.get_mut().set_cookie(buffer_id) };
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

        camera_stream.buffers.push(CameraBuffer {
          buffer_id,
          request: None,
          buffer,
          planes,
        });
      }
      self.streams.push(camera_stream);
    }
    unsafe { self.inner.get_mut().start() }?;
    Ok(true)
  }
  /// Start the process to capture an image from the camera.
  pub fn capture_next_picture(&mut self, stream: usize) -> Result<()> {
    let mut stream = &mut self.streams[stream];
    let buffer = &mut stream.buffers[stream.next_buffer];
    if buffer.request.is_none() {
      let request_id = buffer.buffer_id as u64 | ((stream.stream_id as u64) << 32);
      println!("Queuing request with id {}", request_id);
      let mut req = unsafe { self.inner.get_mut().create_request(request_id) }?;
      unsafe {
        req
          .get_mut()
          .add_buffer(stream.stream.get(), buffer.buffer.get_mut())
      }?;
      for (control_id, control_value) in self.controls.get_libcamera() {
        unsafe { req.get_mut().set_control(control_id, control_value.get()) };
      }
      unsafe { self.inner.get_mut().queue_request(req.get_mut()) }?;
      buffer.request = Some(req);
      stream.next_buffer += 1;
      stream.next_buffer %= stream.buffers.len();
      Ok(())
    } else {
      Err(LibcameraError::NoBufferReady)
    }
  }
  pub fn poll_events(&mut self) -> Result<Vec<CameraEvent>> {
    let events = unsafe { self.inner.get_mut().poll_events() };
    Ok(
      events
        .into_iter()
        .flat_map(|event| match event.message_type {
          ffi::CameraMessageType::RequestComplete => {
            println!("Ev: {event:?}");
            let stream_id = ((event.request_cookie >> 32) & 0xFFFFFFFF) as usize;
            let buffer_id = (event.request_cookie & 0xFFFFFFFF) as usize;
            println!(
              "Request completed on stream {}, buffer {}.",
              stream_id, buffer_id
            );
            let buffer = &mut self.streams[stream_id].buffers[buffer_id];
            buffer.request = None;
            Some(CameraEvent::RequestComplete(
              buffer
                .planes
                .iter()
                .map(|plane| unsafe { plane.get().read_to_vec() })
                .collect(),
            ))
          }
          _ => None,
        })
        .collect(),
    )
  }
}

impl Drop for Camera<'_> {
  fn drop(&mut self) {
    self.streams = Vec::new();
    unsafe { self.inner.get_mut().stop() }.unwrap();
    unsafe { self.inner.get_mut().release() }.unwrap();
  }
}

/// Represents an event from the camera
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CameraEvent {
  /// Triggered when a capture request has completed, containing a vec of the resulting image planes.
  RequestComplete(Vec<Vec<u8>>),
}

/// Represents the result of applying a configuration to a camera.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigStatus {
  /// The configuration was applied to the camera unchanged
  Unchanged,
  /// The configuration was applied to the camera, but some values have been adjusted by the driver to a supported configuration for this camera
  Changed,
}
