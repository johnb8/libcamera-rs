#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindSize new_size(unsigned int width, unsigned int height) {
  BindSize size{
      .inner = std::make_unique<Size>(libcamera::Size(width, height)),
  };
  return size;
}

libcamera::Size Size::into_inner() { return this->inner; }

void Size::set_width(unsigned int width) { this->inner.width = width; }

unsigned int Size::get_width() const { return this->inner.width; }

void Size::set_height(unsigned int height) { this->inner.height = height; }

unsigned int Size::get_height() const { return this->inner.height; }

rust::String Size::raw_to_string() const { return this->inner.toString(); }
