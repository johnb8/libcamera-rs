#pragma once
#include "libcamera/camera_manager.h"
#include "libcamera/camera.h"
#include "rust/cxx.h"

using CameraConfigurationStatus = libcamera::CameraConfiguration::Status;

struct BridgeCamera;

class CameraManager: public libcamera::CameraManager {
   public:
      void start();
      rust::String version();
      rust::Vec<BridgeCamera> cameras() const;
};

std::unique_ptr<CameraManager>
make_camera_manager();

libcamera::Camera&
get_mut_camera(std::shared_ptr<libcamera::Camera>& cam);

std::unique_ptr<libcamera::CameraConfiguration> generate_camera_configuration(libcamera::Camera& cam, const rust::Vec<libcamera::StreamRole>& roles);
