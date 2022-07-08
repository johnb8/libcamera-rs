#include "core.hpp"

#include "libcamera-rs/src/bridge.rs.h"

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
  this->inner->requestCompleted.connect(this, [&](libcamera::Request *req) {
    this->message_mutex.lock();
    this->message_queue.push(CameraMessage{
        .message_type = CameraMessageType::RequestComplete,
        .request_cookie = req->cookie(),
        .buffer_cookie = 0,
    });
    this->message_mutex.unlock();
  });
}

Camera::~Camera() {
  this->inner->bufferCompleted.disconnect();
  this->inner->requestCompleted.disconnect();
}

std::shared_ptr<libcamera::Camera> Camera::into_shared() {
  VALIDATE_POINTERS()

  return this->inner;
}

void Camera::acquire() {
  VALIDATE_POINTERS()

  int ret = this->inner->acquire();
  if (ret < 0) {
    throw error_from_code(-ret);
  }
}

void Camera::release() {
  VALIDATE_POINTERS()

  int ret = this->inner->release();
  if (ret < 0) {
    throw error_from_code(-ret);
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
    throw error_from_code(ENODEV);
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
    throw error_from_code(-ret);
  }
}

BindRequest Camera::create_request(unsigned long cookie) {
  VALIDATE_POINTERS()

  std::unique_ptr<libcamera::Request> req = this->inner->createRequest(cookie);
  if (!req) {
    throw error_from_code(ENODEV);
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
    throw error_from_code(-ret);
  }
}

void Camera::start() {
  VALIDATE_POINTERS()

  int ret = this->inner->start();
  if (ret < 0) {
    throw error_from_code(-ret);
  }
}

void Camera::stop() {
  VALIDATE_POINTERS()

  int ret = this->inner->stop();
  if (ret < 0) {
    throw error_from_code(-ret);
  }
}

rust::Vec<ControlPair> Camera::get_controls() const {
  VALIDATE_POINTERS()

  rust::Vec<ControlPair> controls;
  for (const auto &[control, value] : this->inner->controls()) {
    ControlPair control_pair{
        .id =
            {
                .inner = std::make_unique<ControlId>(control),
            },
        .min =
            {
                .inner = std::make_unique<ControlValue>(value.min()),
            },
        .max =
            {
                .inner = std::make_unique<ControlValue>(value.max()),
            },
        .value =
            {
                .inner = std::make_unique<ControlValue>(value.def()),
            },
    };
    controls.push_back(std::move(control_pair));
  }
  return controls;
}

rust::Vec<CameraMessage> Camera::poll_events() {
  rust::Vec<CameraMessage> messages;
  while (!this->message_queue.empty()) {
    messages.push_back(this->message_queue.front());
    message_queue.pop();
  }
  return messages;
}

rust::Vec<CameraMessage>
Camera::poll_events_with_cookie(unsigned long request_cookie) {
  rust::Vec<CameraMessage> messages;
  while (!this->message_queue.empty()) {
    if (this->message_queue.front().request_cookie == request_cookie) {
      messages.push_back(this->message_queue.front());
      message_queue.pop();
    }
  }
  return messages;
}
