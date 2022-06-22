#include "core.hpp"

#include <libcamera/formats.h>

#include "libcamera-rs/src/bridge.rs.h"

BindPixelFormat get_default_pixel_format(DefaultPixelFormat default_format) {
  const libcamera::PixelFormat *fmt;
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
  if (!fmt) {
    throw BindErrorCode::EFault;
  }
  BindPixelFormat pixel_format{
      .inner = std::make_unique<PixelFormat>(*fmt),
  };
  return pixel_format;
}

libcamera::PixelFormat PixelFormat::into_inner() { return this->inner; }

rust::String PixelFormat::to_string() { return this->inner.toString(); }
