#include "core.hpp"

#include <libcamera/formats.h>

#include "libcamera-rs/src/bridge.rs.h"

BindPixelFormat get_default_pixel_format(DefaultPixelFormat default_format) {
  const libcamera::PixelFormat *fmt = nullptr;
  switch (default_format) {
  case DefaultPixelFormat::R8:
    fmt = &libcamera::formats::R8;
    break;
  case DefaultPixelFormat::Rgb888:
    fmt = &libcamera::formats::RGB888;
    break;
  case DefaultPixelFormat::Rgb565:
    fmt = &libcamera::formats::RGB565;
    break;
  case DefaultPixelFormat::Bgr888:
    fmt = &libcamera::formats::BGR888;
    break;
  case DefaultPixelFormat::Yuyv:
    fmt = &libcamera::formats::YUYV;
    break;
  case DefaultPixelFormat::Yvyu:
    fmt = &libcamera::formats::YVYU;
    break;
  case DefaultPixelFormat::Yuv420:
    fmt = &libcamera::formats::YUV420;
    break;
  case DefaultPixelFormat::Yuv422:
    fmt = &libcamera::formats::YUV422;
    break;
  case DefaultPixelFormat::Mjpeg:
    fmt = &libcamera::formats::MJPEG;
    break;
  }
  if (fmt == nullptr) {
    throw std::runtime_error("Unknown default pixel format.");
  }
  BindPixelFormat pixel_format{
      .inner = std::make_unique<PixelFormat>(*fmt),
  };
  return pixel_format;
}

DefaultPixelFormat PixelFormat::as_default_pixel_format() const {
  if (this->inner == libcamera::formats::R8) {
    return DefaultPixelFormat::R8;
  }
  if (this->inner == libcamera::formats::RGB888) {
    return DefaultPixelFormat::Rgb888;
  }
  if (this->inner == libcamera::formats::RGB565) {
    return DefaultPixelFormat::Rgb565;
  }
  if (this->inner == libcamera::formats::BGR888) {
    return DefaultPixelFormat::Bgr888;
  }
  if (this->inner == libcamera::formats::YUYV) {
    return DefaultPixelFormat::Yuyv;
  }
  if (this->inner == libcamera::formats::YVYU) {
    return DefaultPixelFormat::Yvyu;
  }
  if (this->inner == libcamera::formats::YUV420) {
    return DefaultPixelFormat::Yuv420;
  }
  if (this->inner == libcamera::formats::YUV422) {
    return DefaultPixelFormat::Yuv422;
  }
  if (this->inner == libcamera::formats::MJPEG) {
    return DefaultPixelFormat::Mjpeg;
  }
  throw std::runtime_error("Unknown pixel format.");
}

libcamera::PixelFormat PixelFormat::into_inner() { return this->inner; }

rust::String PixelFormat::raw_to_string() const {
  return this->inner.toString();
}
