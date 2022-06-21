#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

libcamera::Stream *Stream::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner;
}
