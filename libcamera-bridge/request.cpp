#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

void Request::add_buffer(const Stream &stream, FrameBuffer &buffer) {
  VALIDATE_POINTERS()

  int ret = this->inner->addBuffer(stream.into_ptr(), buffer.into_ptr());
  if (ret < 0) {
    throw error_from_code(-ret);
  }
}

libcamera::Request *Request::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner.get();
}

BindControlValue Request::get_control(uint32_t control_id) const {
  VALIDATE_POINTERS()

  libcamera::ControlList &controls = this->inner->controls();

  if (!controls.contains(control_id)) {
    throw std::runtime_error(
        "No control has been set in this request with the specified id.");
  }
  BindControlValue control_value{
      .inner = std::make_unique<ControlValue>(controls.get(control_id)),
  };
  return control_value;
}

void Request::set_control(uint32_t control_, const ControlValue &value) {
  VALIDATE_POINTERS()

  libcamera::ControlList &controls = this->inner->controls();

  controls.set(control_, value.get_inner());
}

rust::String Request::raw_to_string() const {
  VALIDATE_POINTERS()

  return this->inner->toString();
}
