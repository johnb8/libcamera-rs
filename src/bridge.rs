#[cxx::bridge]
pub mod ffi {
    struct BridgeCamera {
        inner: SharedPtr<Camera>,
    }

    #[namespace = "libcamera"]
    #[repr(i32)]
    enum StreamRole {
        Raw,
        StillCapture,
        VideoRecording,
        Viewfinder,
    }

    unsafe extern "C++" {
        include!("libcamera/stream.h");

        #[namespace = "libcamera"]
        type StreamRole;

        include!("libcamera/camera.h");

        #[namespace = "libcamera"]
        type CameraConfiguration;

        #[namespace = "libcamera"]
        type Camera;

        pub fn id(self: &Camera) -> &CxxString;
        pub fn acquire(self: Pin<&mut Camera>) -> i32;

        #[rust_name = "generate_configuration"]
        pub fn generateConfiguration(self: Pin<&mut Camera>, roles: &CxxVector<StreamRole>) -> UniquePtr<CameraConfiguration>;

        include!("libcamera-rs/libcamera-bridge/core.hpp");

        type CameraManager;

        pub fn make_camera_manager() -> UniquePtr<CameraManager>;
        pub fn start(self: Pin<&mut CameraManager>) -> Result<()>;
        pub fn stop(self: Pin<&mut CameraManager>);
        pub fn version(self: Pin<&mut CameraManager>) -> String;
        pub fn cameras(self: &CameraManager) -> Vec<BridgeCamera>;
        pub fn get(self: Pin<&mut CameraManager>, id: &CxxString) -> SharedPtr<Camera>;
    }
}

