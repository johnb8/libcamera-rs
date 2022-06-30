use crate::prelude::{CameraManager, DefaultPixelFormat, StreamRole};

#[test]
fn test_camera() {
  let mut cm = CameraManager::new().unwrap();
  println!("cm: {cm:?}");
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  println!("cam: {cam:?}");
  let conf = cam.generate_config(&[StreamRole::StillCapture]).unwrap();
  println!("conf: {conf:?}");
  let stream = &mut conf.streams_mut()[0];
  stream.set_default_pixel_format(DefaultPixelFormat::Yuyv);
  stream.set_size(640, 480);
  println!("Configuration applied: {:?}", cam.apply_config().unwrap());
  cam.start_stream().unwrap();
  println!("Started stream.");
  cam.capture_next_picture(0).unwrap();
  println!("Capturing single frame...");
  std::thread::sleep(std::time::Duration::from_millis(100));
  //println!("Saving planes...");
}
