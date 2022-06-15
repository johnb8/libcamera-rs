use crate::bridge::MutFromSharedPtr;
use crate::ffi;

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

  let mut config = ffi::generate_camera_configuration(camera.inner.pin_mut(), &roles);

  assert_eq!(
    config.pin_mut().validate(),
    ffi::CameraConfigurationStatus::Valid
  );
}

#[test]
fn init_camera() {
  let mut manager = ffi::make_camera_manager();
  manager.pin_mut().start().unwrap();

  let mut cameras = manager.pin_mut().cameras();
  let camera = &mut cameras[0];

  camera.inner.pin_mut().acquire();

  let roles = vec![ffi::StreamRole::StillCapture];
  let mut config = ffi::generate_camera_configuration(camera.inner.pin_mut(), &roles);

  ffi::set_stream_pixel_format(
    config.pin_mut().at(0),
    ffi::get_default_pixel_format(ffi::DefaultPixelFormat::Rgb888),
  );
  ffi::set_stream_size(config.pin_mut().at(0), 640, 480);
  ffi::set_stream_buffer_count(config.pin_mut().at(0), 4);

  assert_ne!(
    config.pin_mut().validate(),
    ffi::CameraConfigurationStatus::Invalid
  );

  ffi::configure_camera(camera.inner.pin_mut(), config.pin_mut());

  ffi::connect_camera_request_completed(camera.inner.pin_mut(), |_req| {
    println!("Request Completed");
  });

  let _allocator = ffi::make_frame_buffer_allocator(&camera.inner);

  camera.inner.pin_mut().release();
}
