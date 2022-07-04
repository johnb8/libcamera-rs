use crate::prelude::{CameraEvent, CameraManager, DefaultPixelFormat, StreamRole};

#[test]
fn test_camera() {
  let mut cm = CameraManager::new().unwrap();
  println!("cm: {cm:?}");
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  println!("cam: {cam:?}");
  let conf = cam.generate_config(&[StreamRole::StillCapture]).unwrap();
  println!("conf: {conf:?}");
  let stream = &mut conf.streams_mut()[0];
  stream.set_default_pixel_format(DefaultPixelFormat::Mjpeg);
  stream.set_size(640, 480);
  println!("Configuration applied: {:?}", cam.apply_config().unwrap());
  cam.start_stream().unwrap();
  println!("Started stream.");
  println!("Capturing frames...");
  for i in 0..10 {
    cam.capture_next_picture(0).unwrap();
    println!("Capturing image #{}", i);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let events = cam.poll_events().unwrap();
    for event in events {
      match event {
        CameraEvent::RequestComplete(planes) => {
          println!("Got request back with {} image planes", planes.len());
          for (j, plane) in planes.iter().enumerate() {
            let filename = format!("plane_{}_{}.jpeg", i, j);
            std::fs::write(&filename, plane).unwrap();
            println!("Saved plane to '{filename}'.");
          }
        }
      }
    }
  }
  println!("Done!");
}
