#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindStream StreamConfiguration::stream() const {
  BindStream stream{
      .inner = std::make_unique<Stream>(this->inner->stream()),
  };
  return stream;
}

void StreamConfiguration::set_pixel_format(BindPixelFormat pixel_format) {
  VALIDATE_POINTERS()

  this->inner->pixelFormat = pixel_format.inner->into_inner();
}

BindPixelFormat StreamConfiguration::get_pixel_format() const {
  VALIDATE_POINTERS()

  BindPixelFormat pixel_format{
      .inner = std::make_unique<PixelFormat>(this->inner->pixelFormat),
  };
  return pixel_format;
}

void StreamConfiguration::set_size(BindSize size) {

  this->inner->size = size.inner->into_inner();
}

BindSize StreamConfiguration::get_size() const {
  VALIDATE_POINTERS()

  BindSize size{
      .inner = std::make_unique<Size>(this->inner->size),
  };
  return size;
}

void StreamConfiguration::set_buffer_count(size_t buffer_count) {
  VALIDATE_POINTERS()

  this->inner->bufferCount = buffer_count;
}

size_t StreamConfiguration::get_buffer_count() const {
  VALIDATE_POINTERS()

  return this->inner->bufferCount;
}

rust::String StreamConfiguration::raw_to_string() const {
  return this->inner->toString();
}
