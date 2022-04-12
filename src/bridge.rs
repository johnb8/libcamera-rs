#[cxx::bridge]
pub mod ffi {
    struct BridgeCamera {
        inner: SharedPtr<Camera>,
    }

    unsafe extern "C++" {
        include!("libcamera/camera_manager.h");

        #[namespace = "libcamera"]
        type Camera;

        include!("libcamera-rs/libcamera-bridge/core.hpp");

        type CameraManager;

        pub fn make_camera_manager() -> UniquePtr<CameraManager>;
        pub fn start(self: Pin<&mut CameraManager>) -> Result<()>;
        pub fn version(self: Pin<&mut CameraManager>) -> String;
        pub fn cameras(self: &CameraManager) -> Vec<BridgeCamera>;
    }
}

