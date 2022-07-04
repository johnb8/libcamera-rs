#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

libcamera::FrameBuffer *FrameBuffer::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner;
}

rust::Vec<BindFrameBufferPlane> FrameBuffer::planes() const {
  VALIDATE_POINTERS()

  rust::Vec<BindFrameBufferPlane> vec;
  for (const libcamera::FrameBuffer::Plane &plane : this->inner->planes()) {
    BindFrameBufferPlane bind_plane{
        .inner = std::make_unique<FrameBufferPlane>(&plane),
    };

    vec.push_back(std::move(bind_plane));
  }
  return vec;
}

void FrameBuffer::set_cookie(unsigned int cookie) {
  VALIDATE_POINTERS()

  this->inner->setCookie(cookie);
}

unsigned int FrameBuffer::get_cookie() const {
  VALIDATE_POINTERS()

  return this->inner->cookie();
}
