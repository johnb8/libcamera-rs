#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

libcamera::CameraConfiguration *CameraConfiguration::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner.get();
}

size_t CameraConfiguration::size() const {
  VALIDATE_POINTERS()

  return this->inner->size();
}

BindStreamConfiguration CameraConfiguration::at(uint32_t idx) {
  VALIDATE_POINTERS()

  libcamera::StreamConfiguration *str = &this->inner->at(idx);
  if (str == nullptr) {
    throw std::runtime_error("No stream configuration with specified id.");
  }
  BindStreamConfiguration conf{
      .inner = std::make_unique<StreamConfiguration>(str),
  };
  return conf;
}

CameraConfigurationStatus CameraConfiguration::validate() {
  VALIDATE_POINTERS()

  return this->inner->validate();
}
