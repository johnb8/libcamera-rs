#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

#include <bits/stdc++.h>

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
    throw error_from_code(-ret);
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

BindCamera CameraManager::get_camera_by_id(rust::Str rust_id) {
  VALIDATE_POINTERS()

  std::string cam_id = std::string(rust_id);
  std::shared_ptr<libcamera::Camera> cam = this->inner->get(cam_id);
  // C++ header mismatch will cause this to be completely silly
  if (!cam || cam.use_count() > INT_MAX) {
    throw error_from_code(ENODEV);
  }
  BindCamera bind_cam{
      .inner = std::make_unique<Camera>(cam),
  };
  return bind_cam;
}
