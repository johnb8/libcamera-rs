fn main() {
  cxx_build::bridge("src/bridge.rs")
    .file("libcamera-bridge/core.cpp")
    .file("libcamera-bridge/camera_manager.cpp")
    .file("libcamera-bridge/camera.cpp")
    .file("libcamera-bridge/frame_buffers.cpp")
    .file("libcamera-bridge/config.cpp")
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
