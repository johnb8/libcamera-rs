#pragma once

#include <libcamera/camera.h>
#include <libcamera/camera_manager.h>
#include <libcamera/controls.h>
#include <libcamera/framebuffer_allocator.h>
#include <libcamera/request.h>
#include <libcamera/stream.h>
#include <libcamera/version.h>

#include "rust/cxx.h"

struct BindCameraManager;
struct BindCamera;
struct BindCameraConfiguration;
struct BindStreamConfiguration;
struct BindStream;
struct BindFrameBufferAllocator;
struct BindFrameBuffer;
struct BindRequest;

struct CameraConfiguration;
struct Request;

using CameraConfigurationStatus = libcamera::CameraConfiguration::Status;

// Make sure this->inner is non-null
#define VALIDATE_POINTERS()                                                    \
  if (!this->inner) {                                                          \
    throw(BindErrorCode) EFAULT;                                               \
  }

BindCameraManager make_camera_manager();

struct CameraManager {
private:
  std::unique_ptr<libcamera::CameraManager> inner;

public:
  CameraManager(std::unique_ptr<libcamera::CameraManager> inner_)
      : inner{std::move(inner_)} {}

  void start();
  void stop();
  rust::Vec<rust::String> get_camera_ids();
  BindCamera get_camera_by_id(rust::Str id);
};

struct Camera {
private:
  std::shared_ptr<libcamera::Camera> inner;

public:
  Camera(std::shared_ptr<libcamera::Camera> inner_)
      : inner{std::move(inner_)} {}
  std::shared_ptr<libcamera::Camera> into_shared();

  void acquire();
  void release();
  BindCameraConfiguration
      generate_configuration(rust::Slice<const libcamera::StreamRole>);
  void configure(CameraConfiguration &conf);
  BindRequest create_request();
  void queue_request(Request &req);
  void start();
  void stop();
};

struct CameraConfiguration {
private:
  std::unique_ptr<libcamera::CameraConfiguration> inner;

public:
  CameraConfiguration(std::unique_ptr<libcamera::CameraConfiguration> inner_)
      : inner{std::move(inner_)} {}
  libcamera::CameraConfiguration *into_ptr();

  BindStreamConfiguration at(unsigned int idx);
  CameraConfigurationStatus validate();
};

struct StreamConfiguration {
private:
  libcamera::StreamConfiguration *inner;

public:
  StreamConfiguration(libcamera::StreamConfiguration *inner_) : inner(inner_) {}

  BindStream stream();
};

struct Stream {
private:
  libcamera::Stream *inner;

public:
  Stream(libcamera::Stream *inner_) : inner(inner_) {}
  libcamera::Stream *into_ptr();
};

BindFrameBufferAllocator make_frame_buffer_allocator(Camera &camera);

struct FrameBufferAllocator {
private:
  libcamera::FrameBufferAllocator *inner;

public:
  FrameBufferAllocator(libcamera::FrameBufferAllocator *inner_)
      : inner(inner_) {}
  ~FrameBufferAllocator();

  void allocate(Stream &stream);
  void free(Stream &stream);
  rust::Vec<BindFrameBuffer> buffers(Stream &stream);
};

struct FrameBuffer {
private:
  libcamera::FrameBuffer *inner;

public:
  FrameBuffer(libcamera::FrameBuffer *inner_) : inner(inner_) {}
  libcamera::FrameBuffer *into_ptr();
};

struct Request {
private:
  std::unique_ptr<libcamera::Request> inner;

public:
  Request(std::unique_ptr<libcamera::Request> inner_)
      : inner{std::move(inner_)} {}
  libcamera::Request *into_ptr();

  void add_buffer(Stream &stream, FrameBuffer &buffer);
};
