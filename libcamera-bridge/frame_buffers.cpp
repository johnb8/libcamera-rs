#include "./core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

std::unique_ptr<libcamera::FrameBufferAllocator>
make_frame_buffer_allocator(const std::shared_ptr<libcamera::Camera> &cam) {
  return std::make_unique<libcamera::FrameBufferAllocator>(cam);
}

unsigned int
allocate_frame_buffer_stream(const libcamera::FrameBufferAllocator &alloc,
                             libcamera::Stream &stream) {
  int buffers = ((libcamera::FrameBufferAllocator &)alloc).allocate(&stream);

  if (buffers < 0) {
    throw(CameraError)(-buffers);
  }

  return (unsigned int)buffers;
}

void add_request_buffer(libcamera::Request &req, libcamera::Stream &stream,
                        libcamera::FrameBuffer *buffer) {
  req.addBuffer(&stream, buffer);
}

size_t get_allocator_buffer_count(const libcamera::FrameBufferAllocator &alloc,
                                  libcamera::Stream &stream) {
  return alloc.buffers(&stream).size();
}

libcamera::FrameBuffer *
get_allocator_buffer(const libcamera::FrameBufferAllocator &alloc,
                     libcamera::Stream &stream, size_t idx) {
  return alloc.buffers(&stream).at(idx).get();
}
