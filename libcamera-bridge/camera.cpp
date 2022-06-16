#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

libcamera::Camera &get_mut_camera(std::shared_ptr<libcamera::Camera> &cam) {
  return *cam.get();
}

void start_camera(libcamera::Camera &cam, libcamera::ControlList &controls) {
  cam.start(&controls);
}

void queue_camera_request(libcamera::Camera &cam, libcamera::Request &req) {
  cam.queueRequest(&req);
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
    throw(CameraError)(-res);
  }
}

void connect_camera_buffer_completed(
    libcamera::Camera &cam,
    rust::Fn<void(const libcamera::Request &, const libcamera::FrameBuffer &)>
        callback) {
  cam.bufferCompleted.connect(&cam, [&callback](libcamera::Request *request,
                                                libcamera::FrameBuffer *fb) {
    callback(*request, *fb);
  });
}

void connect_camera_request_completed(
    libcamera::Camera &cam,
    rust::Fn<void(const libcamera::Request &)> callback) {
  cam.requestCompleted.connect(
      &cam, [&callback](libcamera::Request *request) { callback(*request); });
}

void connect_camera_disconnected(libcamera::Camera &cam,
                                 rust::Fn<void()> callback) {
  cam.disconnected.connect(&cam, [&callback]() { callback(); });
}
