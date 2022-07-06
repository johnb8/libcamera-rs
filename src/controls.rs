use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::RangeInclusive;

use crate::bridge::{ffi, GetInner};
use crate::{LibcameraError, Result};

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
  ///
  /// It is gurenteed to lie within MinMaxValue::min() and MinMaxValue::max().
  pub fn get_value(&self) -> T {
    self.value
  }
  /// Gets the stored value if it is not equal to the default stored value.
  pub fn get_value_if_changed(&self) -> Option<T> {
    if self.value != self.default {
      Some(self.value)
    } else {
      None
    }
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
  pub contrast: Option<MinMaxValue<f32>>,
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
  pub(crate) fn get_libcamera(&self) -> Vec<(u32, ffi::BindControlValue)> {
    let mut controls = Vec::new();
    if let Some(ae_enable) = &self.ae_enable {
      ae_enable.get_value_if_changed().map(|value| {
        controls.push((1, unsafe { ffi::new_control_value_bool(value) }));
      });
    }
    if let Some(ae_metering_mode) = &self.ae_metering_mode {
      ae_metering_mode.get_value_if_changed().map(|value| {
        controls.push((3, unsafe { ffi::new_control_value_i32(value) }));
      });
    }
    if let Some(ae_constraint_mode) = &self.ae_constraint_mode {
      ae_constraint_mode.get_value_if_changed().map(|value| {
        controls.push((4, unsafe { ffi::new_control_value_i32(value) }));
      });
    }
    if let Some(ae_exposure_mode) = &self.ae_exposure_mode {
      ae_exposure_mode.get_value_if_changed().map(|value| {
        controls.push((5, unsafe { ffi::new_control_value_i32(value) }));
      });
    }
    if let Some(exposure_value) = &self.exposure_value {
      exposure_value.get_value_if_changed().map(|value| {
        controls.push((6, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(exposure_time) = &self.exposure_time {
      exposure_time.get_value_if_changed().map(|value| {
        controls.push((7, unsafe { ffi::new_control_value_i32(value) }));
      });
    }
    if let Some(analogue_gain) = &self.analogue_gain {
      analogue_gain.get_value_if_changed().map(|value| {
        controls.push((8, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(brightness) = &self.brightness {
      brightness.get_value_if_changed().map(|value| {
        controls.push((9, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(contrast) = &self.contrast {
      contrast.get_value_if_changed().map(|value| {
        controls.push((10, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(awb_enable) = &self.awb_enable {
      awb_enable.get_value_if_changed().map(|value| {
        controls.push((12, unsafe { ffi::new_control_value_bool(value) }));
      });
    }
    if let Some(awb_mode) = &self.awb_mode {
      awb_mode.get_value_if_changed().map(|value| {
        controls.push((13, unsafe { ffi::new_control_value_i32(value) }));
      });
    }
    if let Some(colour_gains) = &self.colour_gains {
      colour_gains.get_value_if_changed().map(|value| {
        controls.push((15, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(saturation) = &self.saturation {
      saturation.get_value_if_changed().map(|value| {
        controls.push((17, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(sharpness) = &self.sharpness {
      sharpness.get_value_if_changed().map(|value| {
        controls.push((19, unsafe { ffi::new_control_value_f32(value) }));
      });
    }
    if let Some(colour_correction_matrix) = &self.colour_correction_matrix {
      colour_correction_matrix
        .get_value_if_changed()
        .map(|value| {
          controls.push((21, unsafe { ffi::new_control_value_f32(value) }));
        });
    }
    if let Some(frame_duration_limits) = &self.frame_duration_limits {
      frame_duration_limits.get_value_if_changed().map(|value| {
        controls.push((25, unsafe { ffi::new_control_value_i64(value) }));
      });
    }
    if let Some(noise_reduction_mode) = &self.noise_reduction_mode {
      noise_reduction_mode.get_value_if_changed().map(|value| {
        controls.push((39, unsafe { ffi::new_control_value_i32(value) }));
      });
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
