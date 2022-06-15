#include "./core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

std::unique_ptr<libcamera::FrameBufferAllocator>
make_frame_buffer_allocator(const std::shared_ptr<libcamera::Camera> &cam) {
  return std::make_unique<libcamera::FrameBufferAllocator>(cam);
}

unsigned int
allocate_frame_buffer_stream(libcamera::FrameBufferAllocator &alloc,
                             libcamera::Stream &stream) {
  int buffers = alloc.allocate(&stream);

  if (buffers < 0) {
    throw(CameraError)(-buffers);
  }

  return (unsigned int)buffers;
}
