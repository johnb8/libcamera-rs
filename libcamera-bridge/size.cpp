#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindSize new_size(unsigned int width, unsigned int height) {
  BindSize size{
      .inner = std::make_unique<Size>(libcamera::Size(width, height)),
  };
  return size;
}

libcamera::Size Size::into_inner() { return this->inner; }

unsigned int Size::get_width() const { return this->inner.width; }

unsigned int Size::get_height() const { return this->inner.height; }

void Size::set_width(unsigned int width) { this->inner.width = width; }

void Size::set_height(unsigned int height) { this->inner.height = height; }

rust::String Size::to_string() { return this->inner.toString(); }
