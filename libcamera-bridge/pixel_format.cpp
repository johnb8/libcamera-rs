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
  case DefaultPixelFormat::Yvu422:
    fmt = &libcamera::formats::YVU422;
    break;
  case DefaultPixelFormat::Yuv444:
    fmt = &libcamera::formats::YUV444;
    break;
  case DefaultPixelFormat::Mjpeg:
    fmt = &libcamera::formats::MJPEG;
    break;
  }
  if (fmt == nullptr) {
    // The thrown error is an int and so is copyable.
    // NOLINTNEXTLINE(cert-err09-cpp,cert-err61-cpp)
    throw BindErrorCode::EFault;
  }
  BindPixelFormat pixel_format{
      .inner = std::make_unique<PixelFormat>(*fmt),
  };
  return pixel_format;
}

libcamera::PixelFormat PixelFormat::into_inner() { return this->inner; }

rust::String PixelFormat::raw_to_string() const {
  return this->inner.toString();
}
