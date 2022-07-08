#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindSize new_size(uint32_t width, uint32_t height) {
  BindSize size{
      .inner = std::make_unique<Size>(libcamera::Size(width, height)),
  };
  return size;
}

libcamera::Size Size::into_inner() { return this->inner; }

void Size::set_width(uint32_t width) { this->inner.width = width; }

uint32_t Size::get_width() const { return this->inner.width; }

void Size::set_height(uint32_t height) { this->inner.height = height; }

uint32_t Size::get_height() const { return this->inner.height; }

rust::String Size::raw_to_string() const { return this->inner.toString(); }
