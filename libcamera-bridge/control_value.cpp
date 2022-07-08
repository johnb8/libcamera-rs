#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindControlValue new_control_value_bool(bool value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_u8(unsigned char value) {
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

bool ControlValue::get_bool() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeBool) {
    throw std::runtime_error("Bad type! Expected Bool.");
  }
  return this->inner.get<bool>();
}

uint8_t ControlValue::get_u8() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeByte) {
    throw std::runtime_error("Bad type! Expected Byte.");
  }
  return this->inner.get<uint8_t>();
}

int32_t ControlValue::get_i32() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeInteger32) {
    throw std::runtime_error("Bad type! Expected I32.");
  }
  return this->inner.get<int32_t>();
}

int64_t ControlValue::get_i64() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeInteger64) {
    throw std::runtime_error("Bad type! Expected I64.");
  }
  return this->inner.get<int64_t>();
}

float ControlValue::get_f32() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeFloat) {
    throw std::runtime_error("Bad type! Expected Float.");
  }
  return this->inner.get<float>();
}

rust::String ControlValue::raw_to_string() const {
  return this->inner.toString();
}
