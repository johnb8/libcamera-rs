use crate::bridge::ffi;
use crate::bridge::PinMut;

#[test]
fn it_works() {
  let mut cm = ffi::make_camera_manager();
  unsafe { cm.get().start() };
  let camera_ids = unsafe { cm.get().get_camera_ids() };
  println!("Available Cameras: {camera_ids:?}");
  let mut camera = unsafe { cm.get().get_camera_by_id(&camera_ids[0]) };

  unsafe { camera.get().acquire() };

  let mut config = unsafe {
    camera
      .get()
      .generate_configuration(&[ffi::StreamRole::Viewfinder])
  };

  unsafe { camera.get().configure(config.get()) };

  let mut stream_config = unsafe { config.get().at(0) };

  let mut allocator = unsafe { ffi::make_frame_buffer_allocator(camera.get()) };

  let mut stream = unsafe { stream_config.get().stream() };

  unsafe { allocator.get().allocate(stream.get()) };
  let mut requests = Vec::new();
  for mut buffer in unsafe { allocator.get().buffers(stream.get()) } {
    let mut request = unsafe { camera.get().create_request() };
    unsafe { request.get().add_buffer(stream.get(), buffer.get()) };
    requests.push(request);
  }

  unsafe { camera.get().start() };

  for request in &mut requests {
    unsafe { camera.get().queue_request(request.get()) };
  }

  std::thread::sleep(std::time::Duration::from_millis(1000));

  unsafe { camera.get().stop() };
  unsafe { camera.get().release() };
}
