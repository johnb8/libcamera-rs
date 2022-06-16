#pragma once
#include "libcamera/camera.h"
#include "libcamera/camera_manager.h"
#include "libcamera/framebuffer_allocator.h"
#include "libcamera/stream.h"
#include "rust/cxx.h"

using CameraConfigurationStatus = libcamera::CameraConfiguration::Status;

struct BridgeCamera;
struct BridgeStreamConfiguration;

enum class DefaultPixelFormat;

// Camera Manager

class CameraManager : public libcamera::CameraManager {
public:
  void start();
  rust::String version();
  rust::Vec<BridgeCamera> cameras() const;
};

std::unique_ptr<CameraManager> make_camera_manager();

// Camera

libcamera::Camera &get_mut_camera(std::shared_ptr<libcamera::Camera> &cam);

void start_camera(libcamera::Camera &cam, libcamera::ControlList &controls);

void queue_camera_request(libcamera::Camera &cam, libcamera::Request &req);

std::unique_ptr<libcamera::CameraConfiguration>
generate_camera_configuration(libcamera::Camera &cam,
                              const rust::Vec<libcamera::StreamRole> &roles);
void configure_camera(libcamera::Camera &cam,
                      libcamera::CameraConfiguration &conf);

void connect_camera_buffer_completed(
    libcamera::Camera &cam,
    rust::Fn<void(const libcamera::Request &, const libcamera::FrameBuffer &)>
        callback);
void connect_camera_request_completed(
    libcamera::Camera &cam,
    rust::Fn<void(const libcamera::Request &)> callback);
void connect_camera_disconnected(libcamera::Camera &cam,
                                 rust::Fn<void()> callback);

// Frame Buffers

std::unique_ptr<libcamera::FrameBufferAllocator>
make_frame_buffer_allocator(const std::shared_ptr<libcamera::Camera> &cam);

unsigned int
allocate_frame_buffer_stream(libcamera::FrameBufferAllocator &alloc,
                             libcamera::Stream &stream);

void add_request_buffer(libcamera::Request &req, libcamera::Stream &stream,
                        libcamera::FrameBuffer &buffer);

// Camera Configuration

void set_stream_pixel_format(libcamera::StreamConfiguration &conf,
                             const libcamera::PixelFormat &format);
void set_stream_size(libcamera::StreamConfiguration &conf, unsigned int width,
                     unsigned int height);
void set_stream_buffer_count(libcamera::StreamConfiguration &conf,
                             unsigned int buffers);

libcamera::Stream &
get_stream_from_configuration(libcamera::StreamConfiguration &conf);

std::unique_ptr<libcamera::ControlList> new_control_list();

// Misc. Types

const libcamera::PixelFormat &
get_default_pixel_format(DefaultPixelFormat format);
