# Libcamera-rs
[Libcamera](https://www.libcamera.org/) bindings for [Rust](https://www.rust-lang.org/).

## Example usage to take a bunch of pictures and change some camera parameters:
```rust
use libcamera_rs::prelude::*;

fn main() {
  let mut cm = CameraManager::new().unwrap();
  // Get the first camera from the camera manager.
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  // Generate a configuration with one "viewfinder" optimized stream.
  let conf = cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  let stream = &mut conf.streams_mut()[0];
  // Attempt to assign the stream's pixel format and size.
  stream.set_pixel_format(PixelFormat::Mjpeg);
  stream.set_size(640, 480);
  // Apply the configuration.
  cam.apply_config().unwrap();
  // Start the camera stream.
  cam.start_stream().unwrap();
  // Take 10 pictures
  for i in 0..10 {
    // Change brightness and contrast
    let brightness = cam.get_controls_mut().brightness.as_mut().unwrap();
    brightness.set_value((brightness.get_value() + 0.9).rem_euclid(1.5) - 0.5);
    let contrast = cam.get_controls_mut().contrast.as_mut().unwrap();
    contrast.set_value(contrast.get_value() + 0.1);
    // Queue the capture request
    cam.capture_next_picture(0).unwrap();
    // Wait for a bit.
    std::thread::sleep(std::time::Duration::from_millis(200));
    // Poll events (containing the images)
    let events = cam.poll_events(None).unwrap();
    for event in events {
      match event {
        CameraEvent::RequestComplete {
          serial_id, image, ..
        } => {
          // Reencode the image to PNG and save it.
          let decoded_image = image.try_decode().unwrap();
          let rgb_image = decoded_image
            .as_bgr()
            .unwrap()
            .as_rgb()
            .encode_png()
            .unwrap();
          let filename = format!("image_{serial_id}.png");
          std::fs::write(&filename, rgb_image).unwrap();
        }
        _ => todo!()
      }
    }
  }
}
```

Most functions should be well enough documented with rustdoc.

# NOTE

When cross compiling for raspberry pi, ensure `_GLIBCXX_HAVE_ATOMIC_LOCK_POLICY` is unset in `c++config.h`.
Otherwise the shared pointer to the camera will be misinterpreted by the inner reference counted lock type.
This results in the shared pointer that is returned by libcamera appearing to the main thread to only have 1 reference,
which causes it to be immediately destructed, causing it to be invalid by get_camera is called.
