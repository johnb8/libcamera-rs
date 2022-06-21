#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindMemoryBuffer MemoryBuffer::sub_buffer(size_t offset, size_t length) {
  if (offset > this->length || offset + length > this->length) {
    throw BindErrorCode::EFault;
  }
  BindMemoryBuffer buffer{
      .inner = std::make_unique<MemoryBuffer>(this->pointer + offset, length)};
  return buffer;
}

rust::Vec<unsigned char> MemoryBuffer::read_to_vec() {
  rust::Vec<unsigned char> buf;
  for (size_t i = 0; i < this->length; i++) {
    buf.push_back(this->pointer[i]);
  }
  return buf;
}
