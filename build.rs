fn main() {
  cxx_build::bridge("src/bridge.rs")
    .file("libcamera-bridge/camera_manager.cpp")
    .file("libcamera-bridge/camera.cpp")
    .file("libcamera-bridge/camera_configuration.cpp")
    .file("libcamera-bridge/stream_configuration.cpp")
    .file("libcamera-bridge/pixel_format.cpp")
    .file("libcamera-bridge/size.cpp")
    .file("libcamera-bridge/stream.cpp")
    .file("libcamera-bridge/frame_buffer_allocator.cpp")
    .file("libcamera-bridge/frame_buffer.cpp")
    .file("libcamera-bridge/frame_buffer_plane.cpp")
    .file("libcamera-bridge/fd.cpp")
    .file("libcamera-bridge/memory_buffer.cpp")
    .file("libcamera-bridge/request.cpp")
    .flag_if_supported("-std=c++17")
    .include("/usr/local/include/libcamera")
    .include("libcamera/build/include/libcamera")
    .compile("libcamera-bridge");

  println!("cargo:rerun-if-changed=src/bridge.rs");
  println!("cargo:rerun-if-changed=libcamera-bridge/*.cpp");
  println!("cargo:rerun-if-changed=libcamera-bridge/*.hpp");

  // link libcamera
  println!("cargo:rustc-link-lib=dylib=camera");
  println!("cargo:rustc-link-lib=dylib=camera-base");
}
