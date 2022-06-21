#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

int FrameBufferPlane::get_fd() {
  VALIDATE_POINTERS()

  return this->inner->fd.get();
}

size_t FrameBufferPlane::get_offset() {
  VALIDATE_POINTERS()

  return (size_t)this->inner->offset;
}

size_t FrameBufferPlane::get_length() {
  VALIDATE_POINTERS()

  return (size_t)this->inner->length;
}
