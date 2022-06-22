#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindStream StreamConfiguration::stream() {
  BindStream stream{
      .inner = std::make_unique<Stream>(this->inner->stream()),
  };
  return stream;
}

void StreamConfiguration::set_pixel_format(BindPixelFormat pixel_format) {
  VALIDATE_POINTERS()

  this->inner->pixelFormat = pixel_format.inner->into_inner();
}

BindPixelFormat StreamConfiguration::get_pixel_format() {
  VALIDATE_POINTERS()

  BindPixelFormat pixel_format{
      .inner = std::make_unique<PixelFormat>(this->inner->pixelFormat),
  };
  return pixel_format;
}

void StreamConfiguration::set_size(BindSize size) {

  this->inner->size = size.inner->into_inner();
}

BindSize StreamConfiguration::get_size() {
  VALIDATE_POINTERS()

  BindSize size{
      .inner = std::make_unique<Size>(this->inner->size),
  };
  return size;
}

rust::String StreamConfiguration::to_string() {
  return this->inner->toString();
}
