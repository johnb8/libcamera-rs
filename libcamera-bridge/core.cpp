#include "./core.hpp"
#include "libcamera-rs/src/bridge.rs.h"
#include <memory>

void CameraManager::start() {
  int res = libcamera::CameraManager::start();

  if (res < 0) {
    throw res;
  }

  return;
}

rust::String CameraManager::version() {
  return libcamera::CameraManager::version();
}

rust::Vec<BridgeCamera> CameraManager::cameras() const {
  auto cameras = libcamera::CameraManager::cameras();
  rust::Vec<BridgeCamera> rust_cameras;

  for (auto camera : cameras) {
    rust_cameras.push_back(BridgeCamera{.inner = camera});
  }

  return rust_cameras;
}

std::unique_ptr<CameraManager> make_camera_manager() {
  return std::make_unique<CameraManager>();
}

libcamera::Camera &get_mut_camera(std::shared_ptr<libcamera::Camera> &cam) {
  return *cam.get();
}

std::unique_ptr<libcamera::CameraConfiguration>
generate_camera_configuration(libcamera::Camera &cam,
                              const rust::Vec<libcamera::StreamRole> &roles) {
  std::vector<libcamera::StreamRole> cpp_roles;

  for (auto role : roles) {
    cpp_roles.push_back(role);
  }

  return cam.generateConfiguration(cpp_roles);
}
