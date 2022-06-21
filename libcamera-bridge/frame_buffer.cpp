#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

#include <iostream>

libcamera::FrameBuffer *FrameBuffer::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner;
}
