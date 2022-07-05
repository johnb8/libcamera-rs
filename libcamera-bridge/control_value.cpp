#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindControlValue new_control_value_bool(bool value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_byte(unsigned char value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_i32(int value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_i64(long int value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_f32(float value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_string(rust::String value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(
          libcamera::ControlValue(static_cast<std::string>(value))),
  };
  return control_value;
}

const libcamera::ControlValue &ControlValue::get_inner() const {
  return this->inner;
}

rust::String ControlValue::raw_to_string() const {
  return this->inner.toString();
}
