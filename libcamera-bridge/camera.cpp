#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

std::shared_ptr<libcamera::Camera> Camera::into_shared() {
  VALIDATE_POINTERS()

  return this->inner;
}

void Camera::acquire() {
  VALIDATE_POINTERS()

  int ret = this->inner->acquire();
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

void Camera::release() {
  VALIDATE_POINTERS()

  int ret = this->inner->release();
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

BindCameraConfiguration
Camera::generate_configuration(rust::Slice<const libcamera::StreamRole> roles) {
  VALIDATE_POINTERS()

  std::vector<libcamera::StreamRole> roles_vec;
  for (libcamera::StreamRole role : roles) {
    roles_vec.push_back(role);
  }

  std::unique_ptr<libcamera::CameraConfiguration> conf =
      this->inner->generateConfiguration(roles_vec);
  if (!conf) {
    throw(BindErrorCode) ENODEV;
  }

  BindCameraConfiguration bind_conf{
      .inner = std::make_unique<CameraConfiguration>(std::move(conf)),
  };
  return bind_conf;
}

void Camera::configure(CameraConfiguration &conf) {
  VALIDATE_POINTERS()

  int ret = this->inner->configure(conf.into_ptr());
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

BindRequest Camera::create_request(unsigned long cookie) {
  VALIDATE_POINTERS()

  std::unique_ptr<libcamera::Request> req = this->inner->createRequest(cookie);
  if (!req) {
    throw(BindErrorCode) ENODEV;
  }

  BindRequest bind_req{
      .inner = std::make_unique<Request>(std::move(req)),
  };
  return bind_req;
}

void Camera::queue_request(Request &req) {
  VALIDATE_POINTERS()

  int ret = this->inner->queueRequest(req.into_ptr());
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

void Camera::start() {
  VALIDATE_POINTERS()

  int ret = this->inner->start();
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

void Camera::stop() {
  VALIDATE_POINTERS()

  int ret = this->inner->stop();
  if (ret < 0) {
    throw(BindErrorCode)(-ret);
  }
}

Camera::Camera(std::shared_ptr<libcamera::Camera> inner_)
    : inner{std::move(inner_)} {
  this->inner->bufferCompleted.connect(
      this, [&](libcamera::Request *req, libcamera::FrameBuffer *fb) {
        this->message_mutex.lock();
        this->message_queue.push(CameraMessage{
            .message_type = CameraMessageType::BufferComplete,
            .request_cookie = req->cookie(),
            .buffer_cookie = fb->cookie(),
        });
        this->message_mutex.unlock();
      });
}

rust::Vec<CameraMessage> Camera::poll_events() {
  rust::Vec<CameraMessage> messages;
  while (!this->message_queue.empty()) {
    messages.push_back(this->message_queue.front());
    message_queue.pop();
  }
  return messages;
}
