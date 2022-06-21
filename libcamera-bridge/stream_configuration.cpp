#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindStream StreamConfiguration::stream() {
  BindStream stream{
      .inner = std::make_unique<Stream>(this->inner->stream()),
  };
  return stream;
}
