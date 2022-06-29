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
struct BindPixelFormat;
struct BindSize;
struct BindStream;
struct BindFrameBufferAllocator;
struct BindFrameBuffer;
struct BindFrameBufferPlane;
struct BindFd;
struct BindMemoryBuffer;
struct BindRequest;

struct CameraConfiguration;
struct PixelFormat;
struct Size;
struct Request;
enum class DefaultPixelFormat;

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
  void set_pixel_format(BindPixelFormat pixel_format);
  BindPixelFormat get_pixel_format();
  void set_size(BindSize size);
  BindSize get_size();
  void set_buffer_count(size_t buffer_count);
  size_t get_buffer_count();
  rust::String to_string();
};

BindPixelFormat get_default_pixel_format(DefaultPixelFormat default_format);

struct PixelFormat {
private:
  libcamera::PixelFormat inner;

public:
  PixelFormat(libcamera::PixelFormat inner_) : inner(inner_) {}
  libcamera::PixelFormat into_inner();

  rust::String to_string();
};

BindSize new_size(unsigned int width, unsigned int height);

struct Size {
private:
  libcamera::Size inner;

public:
  Size(libcamera::Size inner_) : inner(inner_) {}
  libcamera::Size into_inner();

  unsigned int get_width() const;
  unsigned int get_height() const;
  void set_width(unsigned int width);
  void set_height(unsigned int height);

  rust::String to_string();
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

  size_t allocate(Stream &stream);
  void free(Stream &stream);
  rust::Vec<BindFrameBuffer> buffers(Stream &stream);
};

struct FrameBuffer {
private:
  libcamera::FrameBuffer *inner;

public:
  FrameBuffer(libcamera::FrameBuffer *inner_) : inner(inner_) {}
  libcamera::FrameBuffer *into_ptr();
  rust::Vec<BindFrameBufferPlane> planes();
};

size_t fd_len(int fd);

struct FrameBufferPlane {
private:
  const libcamera::FrameBuffer::Plane *inner;

public:
  FrameBufferPlane(const libcamera::FrameBuffer::Plane *inner_)
      : inner(inner_) {}

  int get_fd();
  size_t get_offset();
  size_t get_length();
};

// File descriptor functions

size_t fd_len(int fd);
BindMemoryBuffer mmap_plane(int fd, size_t len);

struct MemoryBuffer {
private:
  const unsigned char *pointer;
  size_t length;

public:
  MemoryBuffer(const unsigned char *pointer_, size_t length_)
      : pointer(pointer_), length(length_) {}

  BindMemoryBuffer sub_buffer(size_t offset, size_t length);
  rust::Vec<unsigned char> read_to_vec();
};

struct Request {
private:
  std::unique_ptr<libcamera::Request> inner;

public:
  Request(std::unique_ptr<libcamera::Request> inner_)
      : inner{std::move(inner_)} {}
  libcamera::Request *into_ptr();

  void add_buffer(Stream &stream, FrameBuffer &buffer);
  rust::String to_string();
};
