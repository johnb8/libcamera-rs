#include "./core.hpp"
#include "libcamera-rs/src/bridge.rs.h"
#include "libcamera/formats.h"
#include "libcamera/geometry.h"
#include <memory>

std::unique_ptr<CameraManager> make_camera_manager() {
  return std::make_unique<CameraManager>();
}

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

void configure_camera(libcamera::Camera &cam,
                      libcamera::CameraConfiguration &conf) {
  int res = cam.configure(&conf);

  if (res < 0) {
    throw res;
  }

  return;
}

const libcamera::PixelFormat &
get_default_pixel_format(DefaultPixelFormat format) {
  switch (format) {
  case DefaultPixelFormat::Rgb888:
    return libcamera::formats::RGB888;
  default:
    return libcamera::formats::RGB888;
  }
}

void set_stream_pixel_format(libcamera::StreamConfiguration &conf,
                             const libcamera::PixelFormat &format) {
  conf.pixelFormat = format;
}

void set_stream_size(libcamera::StreamConfiguration &conf, unsigned int width,
                     unsigned int height) {
  conf.size = libcamera::Size(width, height);
}

void set_stream_buffer_count(libcamera::StreamConfiguration &conf,
                             unsigned int buffers) {
  conf.bufferCount = buffers;
}
