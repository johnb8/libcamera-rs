#ifndef _LIBCAMERA_BRIDGE_CORE_HPP
#define _LIBCAMERA_BRIDGE_CORE_HPP

#pragma once

#include <cstring>
#include <mutex>
#include <queue>

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
struct BindControlId;
struct BindControlValue;

struct CameraConfiguration;
struct PixelFormat;
struct Size;
struct Request;
struct ControlValue;
struct ControlPair;

enum class DefaultPixelFormat;
enum class CameraControlType;
enum class CameraMessageType;
struct CameraMessage;
struct ControlRectangle;
struct ControlSize;

using CameraConfigurationStatus = libcamera::CameraConfiguration::Status;

static inline std::runtime_error error_from_code(int code) {
  return std::runtime_error(strerror(code));
}

// Make sure this->inner is non-null
#define VALIDATE_POINTERS()                                                    \
  if (!this->inner) {                                                          \
    throw std::runtime_error("Inner pointer invalid.");                        \
  }

BindCameraManager make_camera_manager();

struct CameraManager {
private:
  std::unique_ptr<libcamera::CameraManager> inner;

public:
  explicit CameraManager(std::unique_ptr<libcamera::CameraManager> inner_)
      : inner{std::move(inner_)} {}

  void start();
  void stop();
  [[nodiscard]] rust::Vec<rust::String> get_camera_ids() const;
  BindCamera get_camera_by_id(rust::Str id);
};

struct Camera {
private:
  std::shared_ptr<libcamera::Camera> inner;

  std::mutex message_mutex;
  std::queue<CameraMessage> message_queue;

  std::unordered_map<uint32_t, const libcamera::ControlId *> controls_by_id;

public:
  explicit Camera(std::shared_ptr<libcamera::Camera> inner_);
  ~Camera();
  std::shared_ptr<libcamera::Camera> into_shared();
  const libcamera::ControlId *get_control_by_id(uint32_t id) const;

  void acquire();
  void release();
  BindCameraConfiguration generate_configuration(
      rust::Slice<const libcamera::StreamRole> /*roles*/);
  void configure(CameraConfiguration &conf);
  BindRequest create_request(uint64_t cookie);
  void queue_request(Request &req);
  void start();
  void stop();
  rust::Vec<ControlPair> get_controls() const;
  rust::Vec<CameraMessage> poll_events();
  rust::Vec<CameraMessage> poll_events_with_cookie(uint64_t request_cookie);
};

struct CameraConfiguration {
private:
  std::unique_ptr<libcamera::CameraConfiguration> inner;

public:
  explicit CameraConfiguration(
      std::unique_ptr<libcamera::CameraConfiguration> inner_)
      : inner{std::move(inner_)} {}
  libcamera::CameraConfiguration *into_ptr();

  size_t size() const;
  BindStreamConfiguration at(uint32_t idx);
  CameraConfigurationStatus validate();
};

struct StreamConfiguration {
private:
  libcamera::StreamConfiguration *inner;

public:
  explicit StreamConfiguration(libcamera::StreamConfiguration *inner_)
      : inner(inner_) {}

  [[nodiscard]] BindStream stream() const;
  void set_pixel_format(BindPixelFormat pixel_format);
  [[nodiscard]] BindPixelFormat get_pixel_format() const;
  void set_size(BindSize size);
  [[nodiscard]] BindSize get_size() const;
  void set_buffer_count(size_t buffer_count);
  [[nodiscard]] size_t get_buffer_count() const;
  [[nodiscard]] rust::String raw_to_string() const;
};

BindPixelFormat get_default_pixel_format(DefaultPixelFormat default_format);

struct PixelFormat {
private:
  libcamera::PixelFormat inner;

public:
  explicit PixelFormat(libcamera::PixelFormat inner_) : inner(inner_) {}
  libcamera::PixelFormat into_inner();

  DefaultPixelFormat as_default_pixel_format() const;
  [[nodiscard]] rust::String raw_to_string() const;
};

BindSize new_size(uint32_t width, uint32_t height);

struct Size {
private:
  libcamera::Size inner;

public:
  explicit Size(libcamera::Size inner_) : inner(inner_) {}
  libcamera::Size into_inner();

  void set_width(uint32_t width);
  [[nodiscard]] uint32_t get_width() const;
  void set_height(uint32_t height);
  [[nodiscard]] uint32_t get_height() const;

  [[nodiscard]] rust::String raw_to_string() const;
};

struct Stream {
private:
  libcamera::Stream *inner;

public:
  explicit Stream(libcamera::Stream *inner_) : inner(inner_) {}
  libcamera::Stream *into_ptr();
  const libcamera::Stream *into_ptr() const;
};

BindFrameBufferAllocator make_frame_buffer_allocator(Camera &camera);

struct FrameBufferAllocator {
private:
  std::unique_ptr<libcamera::FrameBufferAllocator> inner;

public:
  explicit FrameBufferAllocator(
      std::unique_ptr<libcamera::FrameBufferAllocator> inner_)
      : inner{std::move(inner_)} {}

  size_t allocate(Stream &stream);
  void free(Stream &stream);
  rust::Vec<BindFrameBuffer> buffers(Stream &stream) const;
};

struct FrameBuffer {
private:
  libcamera::FrameBuffer *inner;

public:
  explicit FrameBuffer(libcamera::FrameBuffer *inner_) : inner(inner_) {}
  libcamera::FrameBuffer *into_ptr();

  [[nodiscard]] rust::Vec<BindFrameBufferPlane> planes() const;
  void set_cookie(uint64_t cookie);
  uint64_t get_cookie() const;
};

size_t fd_len(int file);

struct FrameBufferPlane {
private:
  const libcamera::FrameBuffer::Plane *inner;

public:
  explicit FrameBufferPlane(const libcamera::FrameBuffer::Plane *inner_)
      : inner(inner_) {}

  [[nodiscard]] int get_fd() const;
  [[nodiscard]] size_t get_offset() const;
  [[nodiscard]] size_t get_length() const;
};

// File descriptor functions

BindMemoryBuffer mmap_plane(int file, size_t len);

struct MemoryBuffer {
private:
  const uint8_t *pointer;
  size_t length;

public:
  MemoryBuffer(const uint8_t *pointer_, size_t length_)
      : pointer(pointer_), length(length_) {}

  BindMemoryBuffer sub_buffer(size_t offset, size_t length);
  [[nodiscard]] rust::Vec<uint8_t> read_to_vec() const;
  size_t get_len() const;
  size_t read_to_mut_slice(rust::Slice<uint8_t> buf) const;
};

struct Request {
private:
  std::unique_ptr<libcamera::Request> inner;

public:
  explicit Request(std::unique_ptr<libcamera::Request> inner_)
      : inner{std::move(inner_)} {}
  libcamera::Request *into_ptr();

  void add_buffer(const Stream &stream, FrameBuffer &buffer);
  BindControlValue get_control(uint32_t id) const;
  void set_control(uint32_t control_, const ControlValue &value);
  [[nodiscard]] rust::String raw_to_string() const;
};

struct ControlId {
private:
  const libcamera::ControlId *inner;

public:
  explicit ControlId(const libcamera::ControlId *inner_) : inner{inner_} {}

  rust::String get_name() const;
  uint32_t get_id() const;
  CameraControlType get_type() const;
};

BindControlValue new_control_value_bool(bool value);
BindControlValue new_control_value_u8(uint8_t value);
BindControlValue new_control_value_i32(int32_t value);
BindControlValue new_control_value_i64(int64_t value);
BindControlValue new_control_value_f32(float value);
BindControlValue
new_control_value_f32_array(rust::Slice<const float> values_rust);
BindControlValue new_control_value_string(rust::Str value);
BindControlValue new_control_value_rectangle(ControlRectangle value);
BindControlValue new_control_value_size(ControlSize value);

struct ControlValue {
private:
  const libcamera::ControlValue inner;

public:
  explicit ControlValue(libcamera::ControlValue inner_) : inner{inner_} {}
  const libcamera::ControlValue &get_inner() const;

  CameraControlType get_type() const;
  bool is_array() const;
  size_t len() const;

  bool get_bool() const;
  uint8_t get_u8() const;
  int32_t get_i32() const;
  int64_t get_i64() const;
  float get_f32() const;
  rust::Vec<float> get_f32_array() const;
  rust::String get_string() const;
  ControlRectangle get_rectangle() const;
  ControlSize get_size() const;

  [[nodiscard]] rust::String raw_to_string() const;
};

#endif
