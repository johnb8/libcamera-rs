#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindMemoryBuffer MemoryBuffer::sub_buffer(size_t offset, size_t length) {
  if (offset > this->length || offset + length > this->length) {
    // The thrown error is an int and so is copyable.
    // NOLINTNEXTLINE(cert-err09-cpp,cert-err61-cpp)
    throw BindErrorCode::EFault;
  }
  BindMemoryBuffer buffer{
      // The point of this class is to safely wrap raw memory pointers.
      // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
      .inner = std::make_unique<MemoryBuffer>(this->pointer + offset, length)};
  return buffer;
}

rust::Vec<unsigned char> MemoryBuffer::read_to_vec() const {
  rust::Vec<unsigned char> buf;
  for (size_t i = 0; i < this->length; i++) {
    // The point of this class is to safely wrap raw memory pointers.
    // NOLINTNEXTLINE(cppcoreguidelines-pro-bounds-pointer-arithmetic)
    buf.push_back(this->pointer[i]);
  }
  return buf;
}
