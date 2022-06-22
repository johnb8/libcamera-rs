#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

void Request::add_buffer(Stream &stream, FrameBuffer &buffer) {
  VALIDATE_POINTERS()

  int ret = this->inner->addBuffer(stream.into_ptr(), buffer.into_ptr());
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

libcamera::Request *Request::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner.get();
}

rust::String Request::to_string() { return this->inner->toString(); }
