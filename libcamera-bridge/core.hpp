#pragma once
#include "libcamera/camera_manager.h"
#include "libcamera/camera.h"
#include "rust/cxx.h"

struct BridgeCamera;

class CameraManager: public libcamera::CameraManager {
   public:
      void start();
      rust::String version();
      rust::Vec<BridgeCamera> cameras() const;
};

std::unique_ptr<CameraManager>
make_camera_manager();
