#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

rust::String ControlId::get_name() const {
  VALIDATE_POINTERS()

  return this->inner->name();
}

unsigned int ControlId::get_id() const {
  VALIDATE_POINTERS()

  return this->inner->id();
}

CameraControlType ControlId::get_type() const {
  VALIDATE_POINTERS()

  return static_cast<CameraControlType>(this->inner->type());
}
