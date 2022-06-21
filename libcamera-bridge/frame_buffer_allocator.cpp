#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindFrameBufferAllocator make_frame_buffer_allocator(Camera &camera) {
  BindFrameBufferAllocator allocator{
      .inner = std::make_unique<FrameBufferAllocator>(
          new libcamera::FrameBufferAllocator(camera.into_shared())),
  };
  return allocator;
}

void FrameBufferAllocator::allocate(Stream &stream) {
  VALIDATE_POINTERS()

  int ret = this->inner->allocate(stream.into_ptr());
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

void FrameBufferAllocator::free(Stream &stream) {
  VALIDATE_POINTERS()

  int ret = this->inner->free(stream.into_ptr());
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

rust::Vec<BindFrameBuffer> FrameBufferAllocator::buffers(Stream &stream) {
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

FrameBufferAllocator::~FrameBufferAllocator() {
  if (this->inner) {
    delete this->inner;
  }
}
