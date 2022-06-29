#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

libcamera::CameraConfiguration *CameraConfiguration::into_ptr() {
  VALIDATE_POINTERS()

  return this->inner.get();
}

BindStreamConfiguration CameraConfiguration::at(unsigned int idx) {
  VALIDATE_POINTERS()

  libcamera::StreamConfiguration *str = &this->inner->at(idx);
  if (str == nullptr) {
    throw(BindErrorCode) ENODEV;
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
