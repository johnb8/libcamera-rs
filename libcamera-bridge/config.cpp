#include "./core.hpp"

#include "libcamera/geometry.h"

void set_stream_pixel_format(libcamera::StreamConfiguration &conf,
                             const libcamera::PixelFormat &format) {
  conf.pixelFormat = format;
}

void set_stream_size(libcamera::StreamConfiguration &conf, unsigned int width,
                     unsigned int height) {
  conf.size = libcamera::Size(width, height);
}

void set_stream_buffer_count(libcamera::StreamConfiguration &conf,
                             unsigned int buffers) {
  conf.bufferCount = buffers;
}

libcamera::Stream &
get_stream_from_configuration(libcamera::StreamConfiguration &conf) {
  return *conf.stream();
}
