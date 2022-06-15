#include "./core.hpp"

std::unique_ptr<libcamera::FrameBufferAllocator>
make_frame_buffer_allocator(const std::shared_ptr<libcamera::Camera> &cam) {
  return std::make_unique<libcamera::FrameBufferAllocator>(cam);
}
