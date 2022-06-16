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

  let mut allocator = ffi::make_frame_buffer_allocator(&camera.inner);

  let stream_config_count = config.pin_mut().size() as u32;
  for i in 0..stream_config_count {
    let stream_config = config.pin_mut().at(i);
    let stream = ffi::get_stream_from_configuration(stream_config);

    let buffer_count = ffi::allocate_frame_buffer_stream(allocator.pin_mut(), stream).unwrap();

    assert_eq!(buffer_count, 4);

    let _request = camera.inner.pin_mut().create_request(0);

    /*
        let buffers = ffi::get_allocator_buffers(&stream);
        for i in 0..buffer_count {
          let buffer = buffers[i];
          request.pin_mut().add_buffer(&mut stream, &mut buffer);
        }
    */
  }

  let mut controls = ffi::new_control_list();
  ffi::start_camera(camera.inner.pin_mut(), controls.pin_mut()).unwrap();

  camera.inner.pin_mut().stop();
  camera.inner.pin_mut().release();
}
