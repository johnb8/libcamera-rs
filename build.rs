const SOURCES: [&str; 15] = [
  "libcamera-bridge/camera_manager.cpp",
  "libcamera-bridge/camera.cpp",
  "libcamera-bridge/camera_configuration.cpp",
  "libcamera-bridge/stream_configuration.cpp",
  "libcamera-bridge/pixel_format.cpp",
  "libcamera-bridge/size.cpp",
  "libcamera-bridge/stream.cpp",
  "libcamera-bridge/frame_buffer_allocator.cpp",
  "libcamera-bridge/frame_buffer.cpp",
  "libcamera-bridge/frame_buffer_plane.cpp",
  "libcamera-bridge/fd.cpp",
  "libcamera-bridge/memory_buffer.cpp",
  "libcamera-bridge/request.cpp",
  "libcamera-bridge/control_id.cpp",
  "libcamera-bridge/control_value.cpp",
];

fn main() {
  println!("cargo:rerun-if-changed=src/bridge.rs");
  println!("cargo:rerun-if-changed=libcamera-bridge/core.hpp");
  for source in &SOURCES {
    println!("cargo:rerun-if-changed={source}");
  }

  cxx_build::bridge("src/bridge.rs")
    .flag_if_supported("-std=c++17")
    .warnings(true)
    .extra_warnings(true)
    .files(SOURCES)
    .include("/usr/local/include/libcamera")
    .include("libcamera/build/include/libcamera")
    .compile("libcamera-bridge");

  // link libcamera
  println!("cargo:rustc-link-search=/usr/local/lib");
  println!("cargo:rustc-link-lib=dylib=camera");
  println!("cargo:rustc-link-lib=dylib=camera-base");
}
