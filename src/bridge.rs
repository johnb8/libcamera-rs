use std::pin::Pin;
use cxx::{SharedPtr, UniquePtr};

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

        pub(crate) fn generate_camera_configuration(cam: Pin<&mut Camera>, roles: &Vec<StreamRole>) -> UniquePtr<CameraConfiguration>;

        include!("libcamera-rs/libcamera-bridge/core.hpp");

        pub fn get_mut_camera(cam: &mut SharedPtr<Camera>) -> Pin<&mut Camera>;

        type CameraManager;

        pub fn make_camera_manager() -> UniquePtr<CameraManager>;
        pub fn start(self: Pin<&mut CameraManager>) -> Result<()>;
        pub fn stop(self: Pin<&mut CameraManager>);
        pub fn version(self: Pin<&mut CameraManager>) -> String;
        pub fn cameras(self: &CameraManager) -> Vec<BridgeCamera>;
        pub fn get(self: Pin<&mut CameraManager>, id: &CxxString) -> SharedPtr<Camera>;
    }
}

impl ffi::Camera {
    pub fn generate_configuration(self: Pin<&mut Self>, roles: &Vec<ffi::StreamRole>) -> UniquePtr<ffi::CameraConfiguration> {
        ffi::generate_camera_configuration(self, roles)
    }
}

pub trait MutFromSharedPtr {
    type Target;

    fn pin_mut(&mut self) -> Pin<&mut Self::Target>;
}

impl MutFromSharedPtr for SharedPtr<ffi::Camera> {
    type Target = ffi::Camera;

    fn pin_mut(&mut self) -> Pin<&mut Self::Target> {
        ffi::get_mut_camera(self)
    }
}
