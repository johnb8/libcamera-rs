use crate::bridge::ffi;
use crate::bridge::GetInner;

use std::collections::HashMap;

#[test]
fn test_unsafe_camera() {
  let mut cm = unsafe { ffi::make_camera_manager() };
  unsafe { cm.get_mut().start() }.unwrap();
  let camera_ids = unsafe { cm.get().get_camera_ids() };
  println!("Available Cameras: {camera_ids:?}");
  let mut camera = unsafe { cm.get_mut().get_camera_by_id(&camera_ids[0]) }.unwrap();

  unsafe { camera.get_mut().acquire() }.unwrap();

  let mut allocator = unsafe { ffi::make_frame_buffer_allocator(camera.get_mut()) };

  let mut config = unsafe {
    camera
      .get_mut()
      .generate_configuration(&[ffi::StreamRole::StillCapture])
  }
  .unwrap();

  let mut stream_config = unsafe { config.get_mut().at(0) }.unwrap();

  unsafe {
    stream_config
      .get_mut()
      .set_pixel_format(ffi::get_default_pixel_format(
        ffi::DefaultPixelFormat::Mjpeg,
      ))
  };

  unsafe { stream_config.get_mut().set_size(ffi::new_size(1280, 720)) };

  let status = unsafe { config.get_mut().validate() };
  if status == ffi::CameraConfigurationStatus::Invalid {
    panic!("Invalid Camera Configuration!");
  }
  if status == ffi::CameraConfigurationStatus::Adjusted {
    println!("Camera Configuration Adjusted.");
  }

  unsafe { stream_config.get_mut().set_buffer_count(1) };

  unsafe { camera.get_mut().configure(config.get_mut()) }.unwrap();

  let controls = unsafe { camera.get().get_controls() };
  for control in &controls {
    println!(
      "Camera control '{}': ID={}, type={:?}, value={}",
      unsafe { control.id.get().get_name() },
      unsafe { control.id.get().get_id() },
      unsafe { control.id.get().get_type() },
      unsafe { control.value.get().raw_to_string() },
    );
  }

  let mut stream = unsafe { stream_config.get().stream() };

  let buffer_count = unsafe { allocator.get_mut().allocate(stream.get_mut()) }.unwrap();
  println!("Buffers: {buffer_count}");

  let mut requests = Vec::new();
  let mut planes = Vec::new();
  for mut buffer in unsafe { allocator.get().buffers(stream.get_mut()) } {
    let mut request = unsafe { camera.get_mut().create_request(69) }.unwrap();

    unsafe {
      request
        .get_mut()
        .set_control(9, ffi::new_control_value_f32(-0.5).get())
    };
    for control in &controls {
      println!(
        "Control '{}' value in current request: {:?}",
        unsafe { control.id.get().get_id() },
        unsafe { request.get().get_control(control.id.get().get_id()) }
          .map(|c| unsafe { c.get().raw_to_string() })
      );
    }

    unsafe { buffer.get_mut().set_cookie(420) };

    let mut mapped_buffers: HashMap<i32, (Option<ffi::BindMemoryBuffer>, usize, usize)> =
      HashMap::new();
    for plane in unsafe { buffer.get().planes() } {
      let fd = unsafe { plane.get().get_fd() };
      let length = mapped_buffers
        .entry(fd)
        .or_insert((None, 0, unsafe { ffi::fd_len(fd) }.unwrap()))
        .2;
      if unsafe { plane.get().get_offset() } + unsafe { plane.get().get_length() } > length {
        panic!(
          "Plane is out of buffer: buffer length = {length}, plane offset = {}, plane length = {}",
          unsafe { plane.get().get_offset() },
          unsafe { plane.get().get_length() }
        );
      }
      let map_len = mapped_buffers[&fd].1;
      mapped_buffers.get_mut(&fd).unwrap().1 =
        map_len.max(unsafe { plane.get().get_offset() } + unsafe { plane.get().get_length() });
    }
    for plane in unsafe { buffer.get().planes() } {
      let fd = unsafe { plane.get().get_fd() };
      let mapped_buffer = mapped_buffers.get_mut(&fd).unwrap();
      if mapped_buffer.0.is_none() {
        mapped_buffer.0 = Some(unsafe { ffi::mmap_plane(fd, mapped_buffer.1) }.unwrap());
      }
      planes.push(
        unsafe {
          mapped_buffer
            .0
            .as_mut()
            .unwrap()
            .get_mut()
            .sub_buffer(plane.get().get_offset(), plane.get().get_length())
        }
        .unwrap(),
      );
    }

    unsafe { request.get_mut().add_buffer(stream.get(), buffer.get_mut()) }.unwrap();
    requests.push(request);
  }

  unsafe { camera.get_mut().start() }.unwrap();

  for request in &mut requests {
    unsafe { camera.get_mut().queue_request(request.get_mut()) }.unwrap();
  }

  std::thread::sleep(std::time::Duration::from_millis(1000));

  println!("Events: {:?}", unsafe { camera.get_mut().poll_events() });

  for (i, plane) in planes.iter_mut().enumerate() {
    std::fs::write(&format!("plane_{i}.jpeg"), unsafe {
      plane.get().read_to_vec()
    })
    .unwrap();
  }

  unsafe { camera.get_mut().stop() }.unwrap();
  unsafe { camera.get_mut().release() }.unwrap();
}
