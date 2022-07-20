#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindControlValue new_control_value_bool(bool value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_u8(uint8_t value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_i32(int32_t value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(value)),
  };
  return control_value;
}

BindControlValue new_control_value_i64(int64_t value) {
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

BindControlValue
new_control_value_f32_array(rust::Slice<const float> values_rust) {
  std::vector<float> values;
  for (float value : values_rust) {
    values.push_back(value);
  }
  libcamera::Span<float> span{values};
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(span)),
  };
  return control_value;
}

BindControlValue new_control_value_string(rust::Str value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(
          libcamera::ControlValue(static_cast<std::string>(value))),
  };
  return control_value;
}

BindControlValue new_control_value_rectangle(ControlRectangle value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(libcamera::ControlValue(
          libcamera::Rectangle(value.x, value.y, value.width, value.height))),
  };
  return control_value;
}

BindControlValue new_control_value_size(ControlSize value) {
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(
          libcamera::ControlValue(libcamera::Size(value.width, value.height))),
  };
  return control_value;
}

const libcamera::ControlValue &ControlValue::get_inner() const {
  return this->inner;
}

CameraControlType ControlValue::get_type() const {
  return static_cast<CameraControlType>(this->inner.type());
}

bool ControlValue::is_array() const { return this->inner.isArray(); }

size_t ControlValue::len() const { return this->inner.numElements(); }

bool ControlValue::get_bool() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeBool ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single Bool.");
  }
  return this->inner.get<bool>();
}

uint8_t ControlValue::get_u8() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeByte ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single Byte.");
  }
  return this->inner.get<uint8_t>();
}

int32_t ControlValue::get_i32() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeInteger32 ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single I32.");
  }
  return this->inner.get<int32_t>();
}

int64_t ControlValue::get_i64() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeInteger64 ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single I64.");
  }
  return this->inner.get<int64_t>();
}

float ControlValue::get_f32() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeFloat ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single Float.");
  }
  return this->inner.get<float>();
}

rust::Vec<float> ControlValue::get_f32_array() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeFloat ||
      !this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Float Array.");
  }
  auto span = this->inner.get<libcamera::Span<const float>>();
  rust::Vec<float> values;
  for (float f : span) {
    values.push_back(f);
  }
  return values;
}

rust::String ControlValue::get_string() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeString ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single String.");
  }
  return static_cast<rust::String>(this->inner.get<std::string>());
}

ControlRectangle ControlValue::get_rectangle() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeRectangle ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single Rectangle.");
  }
  auto rect = this->inner.get<libcamera::Rectangle>();
  ControlRectangle crect{
      .x = rect.x,
      .y = rect.y,
      .width = rect.width,
      .height = rect.height,
  };
  return crect;
}

ControlSize ControlValue::get_size() const {
  if (this->inner.type() != libcamera::ControlType::ControlTypeSize ||
      this->inner.isArray()) {
    throw std::runtime_error("Bad type! Expected Single Size.");
  }
  auto size = this->inner.get<libcamera::Size>();
  ControlSize csize{
      .width = size.width,
      .height = size.height,
  };
  return csize;
}

rust::String ControlValue::raw_to_string() const {
  return this->inner.toString();
}
