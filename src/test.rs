use crate::prelude::{CameraEvent, CameraManager, PixelFormat, StreamRole};

#[cfg(feature = "image")]
#[test]
fn test_camera() {
  let mut cm = CameraManager::new().unwrap();
  println!("cm: {cm:?}");
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  println!("cam: {cam:?}");
  let conf = cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  println!("conf: {conf:?}");
  let stream = &mut conf.streams_mut()[0];
  stream.set_pixel_format(PixelFormat::Mjpeg);
  stream.set_size(640, 480);
  println!("Configuration applied: {:?}", cam.apply_config().unwrap());
  cam.start_stream().unwrap();
  println!("Started stream.");
  println!("Capturing frames...");
  for i in 0..50 {
    let brightness = cam.get_controls_mut().brightness.as_mut().unwrap();
    brightness.set_value((brightness.get_value() + 0.6).rem_euclid(1.5) - 0.5);
    let contrast = cam.get_controls_mut().contrast.as_mut().unwrap();
    contrast.set_value(contrast.get_value() + 0.02);
    cam.capture_next_picture(0).unwrap();
    println!("Capturing image #{}", i);
    std::thread::sleep(std::time::Duration::from_millis(100));
    let events = cam.poll_events().unwrap();
    for event in events {
      match event {
        CameraEvent::RequestComplete {
          serial_id, image, ..
        } => {
          let decoded_image = image.try_decode().unwrap();
          let rgb_image = decoded_image
            .as_bgr()
            .unwrap()
            .as_rgb()
            .unwrap()
            .encode_png()
            .unwrap();
          let filename = format!("image_{serial_id}.png");
          std::fs::write(&filename, rgb_image).unwrap();
          println!("Got responce back for request {serial_id} and saved to {filename}.");
        }
      }
    }
  }
  println!("Done!");
}
