mod bridge;

pub use bridge::ffi;

#[cfg(test)]
mod tests {
    use crate::ffi;

    #[test]
    fn it_works() {
        let mut manager = ffi::make_camera_manager();
        manager.pin_mut().version();
    }
}
