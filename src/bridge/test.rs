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
/// Test initializing camera
/// # Safety
/// Always segfaults.
fn init_camera() {
  let mut manager = ffi::make_camera_manager();
  manager.pin_mut().start().unwrap();

  let mut cameras = manager.pin_mut().cameras();
  let camera = &mut cameras[0];

  assert_eq!(camera.inner.pin_mut().acquire(), 0,);

  let roles = vec![ffi::StreamRole::Viewfinder];
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

  // Allocate and map buffers
  let allocator = ffi::make_frame_buffer_allocator(&camera.inner);
  let stream_config_count = config.pin_mut().size() as u32;
  assert_eq!(stream_config_count, 1);
  let buffer_count = ffi::allocate_frame_buffer_stream(
    allocator.as_ref().unwrap(),
    ffi::get_stream_from_configuration(config.pin_mut().at(0)),
  )
  .unwrap();

  assert_eq!(buffer_count, 4);

  ffi::start_camera(camera.inner.pin_mut()).unwrap();

  let buffer_count = ffi::get_allocator_buffer_count(
    &allocator,
    ffi::get_stream_from_configuration(config.pin_mut().at(0)),
  );
  for buffer_id in 0..buffer_count {
    let buffer = ffi::get_allocator_buffer(
      &allocator,
      ffi::get_stream_from_configuration(config.pin_mut().at(0)),
      buffer_id,
    )
    .unwrap();
    let mut request = camera.inner.pin_mut().create_request(0);
    unsafe {
      println!("Buf: {buffer:?}");
      ffi::add_request_buffer(
        request.pin_mut(),
        ffi::get_stream_from_configuration(config.pin_mut().at(0)),
        buffer,
      );
    }
    // ffi::queue_camera_request(camera.inner.pin_mut(), request.pin_mut()).unwrap();
  }

  ffi::connect_camera_request_completed(camera.inner.pin_mut(), |_req| {
    eprintln!("Request Completed");
  });

  std::thread::sleep(std::time::Duration::from_millis(10000));

  assert_eq!(camera.inner.pin_mut().stop(), 0);
  assert_eq!(camera.inner.pin_mut().release(), 0);
}
