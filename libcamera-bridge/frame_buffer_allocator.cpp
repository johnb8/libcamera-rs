#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindFrameBufferAllocator make_frame_buffer_allocator(Camera &camera) {
  BindFrameBufferAllocator allocator{
      .inner = std::make_unique<FrameBufferAllocator>(
          std::make_unique<libcamera::FrameBufferAllocator>(
              camera.into_shared())),
  };
  return allocator;
}

size_t FrameBufferAllocator::allocate(Stream &stream) {
  VALIDATE_POINTERS()

  int ret = this->inner->allocate(stream.into_ptr());
  if (ret < 0) {
    throw error_from_code(-ret);
  }

  return ret;
}

void FrameBufferAllocator::free(Stream &stream) {
  VALIDATE_POINTERS()

  int ret = this->inner->free(stream.into_ptr());
  if (ret < 0) {
    throw error_from_code(-ret);
  }
}

rust::Vec<BindFrameBuffer> FrameBufferAllocator::buffers(Stream &stream) const {
  VALIDATE_POINTERS()

  rust::Vec<BindFrameBuffer> vec;
  for (const std::unique_ptr<libcamera::FrameBuffer> &buffer :
       this->inner->buffers(stream.into_ptr())) {
    BindFrameBuffer bind_buffer{
        .inner = std::make_unique<FrameBuffer>(buffer.get()),
    };

    vec.push_back(std::move(bind_buffer));
  }
  return vec;
}
