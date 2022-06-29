#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

BindCameraManager make_camera_manager() {
  BindCameraManager manager{
      .inner = std::make_unique<CameraManager>(
          std::make_unique<libcamera::CameraManager>()),
  };
  return manager;
}

void CameraManager::start() {
  VALIDATE_POINTERS()

  int ret = this->inner->start();
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

void CameraManager::stop() {
  VALIDATE_POINTERS()

  this->inner->stop();
}

rust::Vec<rust::String> CameraManager::get_camera_ids() const {
  VALIDATE_POINTERS()

  rust::Vec<rust::String> camera_ids;
  for (std::shared_ptr<libcamera::Camera> cam : this->inner->cameras()) {
    camera_ids.push_back(cam->id());
  }
  return camera_ids;
}

BindCamera CameraManager::get_camera_by_id(rust::Str id) {
  VALIDATE_POINTERS()

  std::shared_ptr<libcamera::Camera> cam = this->inner->get(std::string(id));
  if (!cam) {
    throw(BindErrorCode) ENODEV;
  }
  BindCamera bind_cam{
      .inner = std::make_unique<Camera>(cam),
  };
  return bind_cam;
}
