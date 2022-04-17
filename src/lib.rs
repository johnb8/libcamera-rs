mod bridge;

pub use bridge::ffi;

#[cfg(test)]
mod tests {
    use crate::ffi;
    use crate::bridge::MutFromSharedPtr;

    #[test]
    fn it_works() {
        let mut manager = ffi::make_camera_manager();
        manager.pin_mut().version();
    }

    #[test]
    fn generate_configuration() {
        let mut manager = ffi::make_camera_manager();
        manager.pin_mut().start().unwrap();

        let mut cameras = manager.pin_mut().cameras();
        let camera = &mut cameras[0];

        let roles = vec![ffi::StreamRole::StillCapture];

        camera.inner.pin_mut().generate_configuration(&roles);
    }
}
