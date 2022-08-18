use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::RangeInclusive;

use log::{error, trace, warn};

use crate::bridge::{ffi, GetInner};
use crate::{LibcameraError, Result};

/// Contains a value with an acceptable minimum and maximum and a default.
#[derive(Debug)]
pub struct MinMaxValue<T: PartialOrd + Debug> {
  range: Option<RangeInclusive<T>>,
  default: T,
  value: T,
}

/// A pair of two float values.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatPair(pub f32, pub f32);

/// Things that can be clamped to a range.
pub trait Clampable {
  /// Clamp self to fit inside range.
  fn clamp(self, range: &RangeInclusive<Self>) -> Self
  where
    Self: Sized + PartialOrd + Clone,
  {
    if range.start() > &self {
      range.start().clone()
    } else if range.end() < &self {
      range.end().clone()
    } else {
      self
    }
  }
}

impl Clampable for bool {}
impl Clampable for u8 {}
impl Clampable for i32 {}
impl Clampable for i64 {}
impl Clampable for f32 {}
impl Clampable for String {}

impl Clampable for FloatPair {
  fn clamp(self, range: &RangeInclusive<Self>) -> Self {
    FloatPair(
      self.0.clamp(range.start().0, range.end().0),
      self.1.clamp(range.start().1, range.end().1),
    )
  }
}

/// Represents a control value rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
  /// The starting x position
  x: i32,
  /// The starting y position
  y: i32,
  /// The width
  width: u32,
  /// The height
  height: u32,
}

impl From<ffi::ControlRectangle> for Rectangle {
  fn from(v: ffi::ControlRectangle) -> Rectangle {
    Rectangle {
      x: v.x,
      y: v.y,
      width: v.width,
      height: v.height,
    }
  }
}

impl From<Rectangle> for ffi::ControlRectangle {
  fn from(v: Rectangle) -> ffi::ControlRectangle {
    ffi::ControlRectangle {
      x: v.x,
      y: v.y,
      width: v.width,
      height: v.height,
    }
  }
}

impl PartialOrd for Rectangle {
  fn partial_cmp(&self, other: &Rectangle) -> Option<Ordering> {
    let x = self.x.cmp(&other.x);
    let y = self.y.cmp(&other.y);
    let w = self.width.cmp(&other.width);
    let h = self.height.cmp(&other.height);
    if (x == y && w == h && x == w) || (y == Ordering::Equal && w == y && h == y) {
      Some(x)
    } else if x == Ordering::Equal && w == x && h == x {
      Some(y)
    } else if x == Ordering::Equal && y == x && h == x {
      Some(w)
    } else if x == Ordering::Equal && y == x && w == x {
      Some(h)
    } else {
      None
    }
  }
}

impl Clampable for Rectangle {
  fn clamp(self, range: &RangeInclusive<Self>) -> Self {
    Rectangle {
      x: Ord::clamp(self.x, range.start().x, range.end().x),
      y: Ord::clamp(self.y, range.start().y, range.end().y),
      width: Ord::clamp(self.width, range.start().width, range.end().height),
      height: Ord::clamp(self.height, range.start().width, range.end().height),
    }
  }
}

/// Represents a control value size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
  /// The width.
  width: u32,
  /// The height.
  height: u32,
}

impl From<ffi::ControlSize> for Size {
  fn from(v: ffi::ControlSize) -> Size {
    Size {
      width: v.width,
      height: v.height,
    }
  }
}

impl From<Size> for ffi::ControlSize {
  fn from(v: Size) -> ffi::ControlSize {
    ffi::ControlSize {
      width: v.width,
      height: v.height,
    }
  }
}

impl Clampable for Size {
  fn clamp(self, range: &RangeInclusive<Self>) -> Self {
    Size {
      width: Ord::clamp(self.width, range.start().width, range.end().height),
      height: Ord::clamp(self.height, range.start().width, range.end().height),
    }
  }
}

impl PartialOrd for Size {
  fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
    let w = self.width.cmp(&other.width);
    let h = self.height.cmp(&other.height);
    if w == h || h == Ordering::Equal {
      Some(w)
    } else if w == Ordering::Equal {
      Some(h)
    } else {
      None
    }
  }
}

impl<T: 'static + PartialOrd + Clampable + Clone + Debug + Sized + Send + Sync> MinMaxValue<T> {
  /// Creates a new MinMaxValue out of a given min, max, and default
  ///
  /// # Returns
  /// Returns None if default is not within min and max.
  pub fn new(min: T, max: T, default: T) -> Result<MinMaxValue<T>> {
    if min >= max {
      return Ok(MinMaxValue {
        range: None,
        value: default.clone(),
        default,
      });
    }
    let range = min..=max;
    if range.contains(&default) {
      Ok(MinMaxValue {
        range: Some(range),
        value: default.clone(),
        default,
      })
    } else {
      Err(LibcameraError::InvalidControlValue(Box::new(default)))
    }
  }
  /// Retrieve the default value
  pub fn get_default(&self) -> &T {
    &self.default
  }
  /// Retrieve the minimum value
  pub fn min(&self) -> Option<&T> {
    self.range.as_ref().map(|r| r.start())
  }
  /// Retrieve the maximum value
  pub fn max(&self) -> Option<&T> {
    self.range.as_ref().map(|r| r.end())
  }
  /// Gets the stored value
  ///
  /// It is gurenteed to lie within MinMaxValue::min() and MinMaxValue::max().
  pub fn get_value(&self) -> &T {
    &self.value
  }
  /// Gets the stored value if it is not equal to the default stored value.
  pub fn get_value_if_changed(&self) -> Option<&T> {
    if self.value != self.default {
      Some(&self.value)
    } else {
      None
    }
  }
  /// Verifies that value lies within the acceptable range for this value, then sets this value.
  ///
  /// # Returns
  /// `true` if the value lies within the acceptable range for this value and was stored, `false` otherwise.
  pub fn set_value(&mut self, value: T) -> bool {
    if let Some(range) = &self.range {
      if range.contains(&value) {
        self.value = value;
        true
      } else {
        false
      }
    } else {
      self.value = value;
      true
    }
  }
  /// Set this value to the given value.
  pub fn set_value_clamped(&mut self, value: T) {
    if let Some(range) = &self.range {
      self.value = value.clamp(range)
    } else {
      self.value = value;
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

impl<
    T: 'static
      + TryFrom<i32, Error = LibcameraError>
      + ControlEnum
      + Clampable
      + Copy
      + Debug
      + PartialOrd
      + Send
      + Sync,
  > TryFrom<&ffi::ControlPair> for MinMaxValue<T>
{
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<T>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_i32() }?.try_into()?,
      unsafe { pair.max.get().get_i32() }?.try_into()?,
      unsafe { pair.value.get().get_i32() }?.try_into()?,
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

impl TryFrom<Vec<f32>> for FloatPair {
  type Error = LibcameraError;
  fn try_from(arr: Vec<f32>) -> Result<FloatPair> {
    if let &[a1, a2] = &arr[..] {
      Ok(FloatPair(a1, a2))
    } else {
      Err(LibcameraError::ControlValueError)
    }
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<FloatPair> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<FloatPair>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_f32_array() }
        .map_err(LibcameraError::InnerError)
        .and_then(|v| v.try_into())
        .or_else(|_| unsafe { pair.min.get().get_f32() }.map(|v| FloatPair(v, v)))?,
      unsafe { pair.max.get().get_f32_array() }
        .map_err(LibcameraError::InnerError)
        .and_then(|v| v.try_into())
        .or_else(|_| unsafe { pair.min.get().get_f32() }.map(|v| FloatPair(v, v)))?,
      unsafe { pair.value.get().get_f32_array() }
        .map_err(LibcameraError::InnerError)
        .and_then(|v| v.try_into())
        .or_else(|_| unsafe { pair.min.get().get_f32() }.map(|v| FloatPair(v, v)))?,
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<String> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<String>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_string() }?,
      unsafe { pair.max.get().get_string() }?,
      unsafe { pair.value.get().get_string() }?,
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<Rectangle> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<Rectangle>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_rectangle() }?.into(),
      unsafe { pair.max.get().get_rectangle() }?.into(),
      unsafe { pair.value.get().get_rectangle() }?.into(),
    )
  }
}

impl TryFrom<&ffi::ControlPair> for MinMaxValue<Size> {
  type Error = LibcameraError;
  fn try_from(pair: &ffi::ControlPair) -> Result<MinMaxValue<Size>> {
    MinMaxValue::new(
      unsafe { pair.min.get().get_size() }?.into(),
      unsafe { pair.max.get().get_size() }?.into(),
      unsafe { pair.value.get().get_size() }?.into(),
    )
  }
}

/// Represents a camera control value with an unknown type
///
/// Most of the time you probably want to use `CameraControls` instead.
#[non_exhaustive]
#[derive(Debug)]
pub enum CameraControlValue {
  /// A control value not containing a value.
  None,
  /// A control value containing a boolean, e.g. autoexpose enable.
  Bool(MinMaxValue<bool>),
  /// A control value containing a single byte value.
  Byte(MinMaxValue<u8>),
  /// A control value containing a 32-bit integer, e.g. exposure time.
  Integer32(MinMaxValue<i32>),
  /// A control value containing a 64-bit integer, e.g. frame duration limit.
  Integer64(MinMaxValue<i64>),
  /// A control value containing a 32-bit float, e.g. brightness.
  Float(MinMaxValue<f32>),
  /// A control value containing an array of 32-bit floats.
  FloatArray(Vec<MinMaxValue<f32>>),
  /// A control value containing a String.
  String(MinMaxValue<String>),
  /// A control value containing a Rectangle
  Rectangle(MinMaxValue<Rectangle>),
  /// A control value containing a Size.
  Size(MinMaxValue<Size>),
}

/// Camera auto exposure metering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AeMeteringMode {
  /// Hah brittish spelling
  CentreWeighted,
  /// Spot
  Spot,
  /// Oooh fancy sounding
  Matrix,
  /// ???
  Custom,
}

/// Camera auto exposure constraint mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AeConstraintMode {
  /// Normal
  Normal,
  /// Highlights???
  Highlight,
  /// ???
  Custom,
}

/// Camera auto exposure mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AeExposureMode {
  /// Normal
  Normal,
  /// Shorter than normal
  Short,
  /// Longer than normal
  Long,
}

/// Camera auto white balance mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AwbMode {
  /// Auto
  Auto,
  /// Incandescent
  Incandescent,
  /// Tungsten
  Tungsten,
  /// Fluorescent
  Fluorescent,
  /// Indoor
  Indoor,
  /// Daylight
  Daylight,
  /// Cloudy
  Cloudy,
  /// Custom
  Custom,
}

trait ControlEnum {}

impl ControlEnum for AeMeteringMode {}
impl ControlEnum for AeConstraintMode {}
impl ControlEnum for AeExposureMode {}
impl ControlEnum for AwbMode {}

impl Clampable for AeMeteringMode {}
impl Clampable for AeConstraintMode {}
impl Clampable for AeExposureMode {}
impl Clampable for AwbMode {}

impl TryFrom<i32> for AeMeteringMode {
  type Error = LibcameraError;
  fn try_from(i: i32) -> Result<Self> {
    match i {
      0 => Ok(Self::CentreWeighted),
      1 => Ok(Self::Spot),
      2 => Ok(Self::Matrix),
      3 => Ok(Self::Custom),
      i => Err(LibcameraError::InvalidControlValue(Box::new(i))),
    }
  }
}
impl TryFrom<i32> for AeConstraintMode {
  type Error = LibcameraError;
  fn try_from(i: i32) -> Result<Self> {
    match i {
      0 => Ok(Self::Normal),
      1 => Ok(Self::Highlight),
      2 => Ok(Self::Custom),
      i => Err(LibcameraError::InvalidControlValue(Box::new(i))),
    }
  }
}
impl TryFrom<i32> for AeExposureMode {
  type Error = LibcameraError;
  fn try_from(i: i32) -> Result<Self> {
    match i {
      0 => Ok(Self::Normal),
      1 => Ok(Self::Short),
      2 => Ok(Self::Long),
      i => Err(LibcameraError::InvalidControlValue(Box::new(i))),
    }
  }
}
impl TryFrom<i32> for AwbMode {
  type Error = LibcameraError;
  fn try_from(i: i32) -> Result<Self> {
    match i {
      0 => Ok(Self::Auto),
      1 => Ok(Self::Incandescent),
      2 => Ok(Self::Tungsten),
      3 => Ok(Self::Fluorescent),
      4 => Ok(Self::Indoor),
      5 => Ok(Self::Daylight),
      6 => Ok(Self::Cloudy),
      7 => Ok(Self::Custom),
      i => Err(LibcameraError::InvalidControlValue(Box::new(i))),
    }
  }
}

/// Stores camera controls.
///
/// Common controls are fields on this struct
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct CameraControls {
  /// Autoexposure enable.
  pub ae_enable: Option<MinMaxValue<bool>>,
  /// Autoexposure metering mode.
  pub ae_metering_mode: Option<MinMaxValue<AeMeteringMode>>,
  /// Autoexposure constraint mode.
  pub ae_constraint_mode: Option<MinMaxValue<AeConstraintMode>>,
  /// Autoexposure mode.
  pub ae_exposure_mode: Option<MinMaxValue<AeExposureMode>>,
  /// Exposure "value".
  pub exposure_value: Option<MinMaxValue<f32>>,
  /// Exposure time.
  pub exposure_time: Option<MinMaxValue<i32>>,
  /// Analogue signal gain.
  pub analogue_gain: Option<MinMaxValue<f32>>,
  /// Brightness
  pub brightness: Option<MinMaxValue<f32>>,
  /// Contrast
  pub contrast: Option<MinMaxValue<f32>>,
  /// Auto white balance enable.
  pub awb_enable: Option<MinMaxValue<bool>>,
  /// Auto white balance mode.
  pub awb_mode: Option<MinMaxValue<AwbMode>>,
  /// Red/Blue colour gains.
  pub colour_gains: Option<MinMaxValue<FloatPair>>,
  /// Saturation.
  pub saturation: Option<MinMaxValue<f32>>,
  /// Sharpness.
  pub sharpness: Option<MinMaxValue<f32>>,
  /// Colour correction matrix.
  /// **TODO**: Make this actually a 3x3 matrix
  pub colour_correction_matrix: Option<MinMaxValue<f32>>,
  /// Scaler crop
  pub scaler_crop: Option<MinMaxValue<Rectangle>>, // Rectangle TODO
  /// Frame duration limit.
  pub frame_duration_limits: Option<MinMaxValue<i64>>,
  /// Noise reduction mode.
  /// **TODO**: This should be an enum.
  pub noise_reduction_mode: Option<MinMaxValue<i32>>,
  /// Values not directly handled by this struct but found on your camera, maps control IDs to a tuple containing a name for the control as well as the value.
  pub others: HashMap<u32, (String, CameraControlValue)>,
}

impl CameraControls {
  pub(crate) fn from_libcamera(control_list: Vec<ffi::ControlPair>) -> Self {
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
        "Contrast" => (&control)
          .try_into()
          .map(|control| controls.contrast = Some(control))
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
        let control_type = unsafe { control.id.get().get_type() };
        let control_array = unsafe { control.value.get().is_array() };
        let control_value = match (control_type, control_array) {
          (ffi::CameraControlType::None, false) => Some(Ok(CameraControlValue::None)),
          (ffi::CameraControlType::Bool, false) => {
            Some((&control).try_into().map(CameraControlValue::Bool))
          }
          (ffi::CameraControlType::Byte, false) => {
            Some((&control).try_into().map(CameraControlValue::Byte))
          }
          (ffi::CameraControlType::Integer32, false) => {
            Some((&control).try_into().map(CameraControlValue::Integer32))
          }
          (ffi::CameraControlType::Integer64, false) => {
            Some((&control).try_into().map(CameraControlValue::Integer64))
          }
          (ffi::CameraControlType::Float, false) => {
            Some((&control).try_into().map(CameraControlValue::Float))
          }
          _ => None,
        };
        match control_value {
          Some(Ok(control_value)) => {
            controls
              .others
              .insert(unsafe { control.id.get().get_id() }, (name, control_value));
          }
          Some(Err(e)) => error!("Camera control with conflicting types: {name} is supposed to have type of {control_type:?}, err: {e}"),
          None => warn!("Unknown type for camera control {name}."),
        };
      }
    }
    controls
  }
  pub(crate) fn get_libcamera(&self) -> Vec<(u32, ffi::BindControlValue)> {
    let mut controls = Vec::new();
    if let Some(ae_enable) = &self.ae_enable {
      if let Some(&value) = ae_enable.get_value_if_changed() {
        controls.push((1, unsafe { ffi::new_control_value_bool(value) }));
      }
    }
    if let Some(ae_metering_mode) = &self.ae_metering_mode {
      if let Some(&value) = ae_metering_mode.get_value_if_changed() {
        controls.push((3, unsafe { ffi::new_control_value_i32(value as i32) }));
      }
    }
    if let Some(ae_constraint_mode) = &self.ae_constraint_mode {
      if let Some(&value) = ae_constraint_mode.get_value_if_changed() {
        controls.push((4, unsafe { ffi::new_control_value_i32(value as i32) }));
      }
    }
    if let Some(ae_exposure_mode) = &self.ae_exposure_mode {
      if let Some(&value) = ae_exposure_mode.get_value_if_changed() {
        controls.push((5, unsafe { ffi::new_control_value_i32(value as i32) }));
      }
    }
    if let Some(exposure_value) = &self.exposure_value {
      if let Some(&value) = exposure_value.get_value_if_changed() {
        controls.push((6, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(exposure_time) = &self.exposure_time {
      if let Some(&value) = exposure_time.get_value_if_changed() {
        controls.push((7, unsafe { ffi::new_control_value_i32(value) }));
      }
    }
    if let Some(analogue_gain) = &self.analogue_gain {
      if let Some(&value) = analogue_gain.get_value_if_changed() {
        controls.push((8, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(brightness) = &self.brightness {
      if let Some(&value) = brightness.get_value_if_changed() {
        controls.push((9, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(contrast) = &self.contrast {
      if let Some(&value) = contrast.get_value_if_changed() {
        controls.push((10, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(awb_enable) = &self.awb_enable {
      if let Some(&value) = awb_enable.get_value_if_changed() {
        controls.push((12, unsafe { ffi::new_control_value_bool(value) }));
      }
    }
    if let Some(awb_mode) = &self.awb_mode {
      if let Some(&value) = awb_mode.get_value_if_changed() {
        controls.push((13, unsafe { ffi::new_control_value_i32(value as i32) }));
      }
    }
    if let Some(colour_gains) = &self.colour_gains {
      if let Some(&value) = colour_gains.get_value_if_changed() {
        controls.push((15, unsafe {
          ffi::new_control_value_f32_array(&[value.0, value.1])
        }));
      }
    }
    if let Some(saturation) = &self.saturation {
      if let Some(&value) = saturation.get_value_if_changed() {
        controls.push((17, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(sharpness) = &self.sharpness {
      if let Some(&value) = sharpness.get_value_if_changed() {
        controls.push((19, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(colour_correction_matrix) = &self.colour_correction_matrix {
      if let Some(&value) = colour_correction_matrix.get_value_if_changed() {
        controls.push((21, unsafe { ffi::new_control_value_f32(value) }));
      }
    }
    if let Some(frame_duration_limits) = &self.frame_duration_limits {
      if let Some(&value) = frame_duration_limits.get_value_if_changed() {
        controls.push((25, unsafe { ffi::new_control_value_i64(value) }));
      }
    }
    if let Some(noise_reduction_mode) = &self.noise_reduction_mode {
      if let Some(&value) = noise_reduction_mode.get_value_if_changed() {
        controls.push((39, unsafe { ffi::new_control_value_i32(value) }));
      }
    }
    for (id, (_name, value)) in &self.others {
      if let Some(value) = match value {
        CameraControlValue::None => None,
        CameraControlValue::Bool(value) => {
          Some(unsafe { ffi::new_control_value_bool(*value.get_value()) })
        }
        CameraControlValue::Byte(value) => {
          Some(unsafe { ffi::new_control_value_u8(*value.get_value()) })
        }
        CameraControlValue::Integer32(value) => {
          Some(unsafe { ffi::new_control_value_i32(*value.get_value()) })
        }
        CameraControlValue::Integer64(value) => {
          Some(unsafe { ffi::new_control_value_i64(*value.get_value()) })
        }
        CameraControlValue::Float(value) => {
          Some(unsafe { ffi::new_control_value_f32(*value.get_value()) })
        }
        CameraControlValue::FloatArray(value) => Some(unsafe {
          ffi::new_control_value_f32_array(
            value
              .iter()
              .map(|v| *v.get_value())
              .collect::<Vec<_>>()
              .as_slice(),
          )
        }),
        CameraControlValue::String(value) => {
          Some(unsafe { ffi::new_control_value_string(value.get_value()) })
        }
        CameraControlValue::Rectangle(value) => Some(unsafe {
          ffi::new_control_value_rectangle(ffi::ControlRectangle::from(*value.get_value()))
        }),
        CameraControlValue::Size(value) => {
          Some(unsafe { ffi::new_control_value_size(ffi::ControlSize::from(*value.get_value())) })
        }
      } {
        controls.push((*id, value));
      }
    }
    controls
  }
}
