#include <memory>
#include "./core.hpp"
#include "libcamera-rs/src/bridge.rs.h"

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
        rust_cameras.push_back(BridgeCamera { .inner = camera });
    }

    return rust_cameras;
}

std::unique_ptr<CameraManager>
make_camera_manager() {
  return std::make_unique<CameraManager>();
}
