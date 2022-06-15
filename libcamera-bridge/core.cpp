#include "./core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

#include "libcamera/formats.h"

const libcamera::PixelFormat &
get_default_pixel_format(DefaultPixelFormat format) {
  switch (format) {
  case DefaultPixelFormat::Rgb888:
    return libcamera::formats::RGB888;
  default:
    return libcamera::formats::RGB888;
  }
}
