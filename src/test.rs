use crate::prelude::{
  CameraEvent, CameraImage, CameraManager, ConfigStatus, LibcameraError, PixelFormat, StreamRole,
};

#[cfg(feature = "image")]
#[test]
fn test_camera() {
  let cm = CameraManager::new().unwrap();
  println!("cm: {cm:?}");
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  println!("cam: {cam:?}");
  let conf = cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  println!("conf: {conf:?}");
  let stream = &mut conf.streams_mut()[0];
  stream.set_pixel_format(PixelFormat::Yuv420);
  stream.set_size(640, 480);
  println!("Configuration applied: {:?}", cam.apply_config().unwrap());
  cam.start_stream().unwrap();
  println!("Started stream.");
  println!("Capturing frames...");
  for i in 0..6 {
    let brightness = cam.get_controls_mut().brightness.as_mut().unwrap();
    brightness.set_value((brightness.get_value() + 0.6).rem_euclid(1.5) - 0.5);
    let contrast = cam.get_controls_mut().contrast.as_mut().unwrap();
    contrast.set_value(contrast.get_value() + 0.02);
    cam.capture_next_picture(0).unwrap();
    println!("Capturing image #{}", i);
    std::thread::sleep(std::time::Duration::from_millis(200));
    let events = cam.poll_events(None).unwrap();
    for event in events {
      match event {
        CameraEvent::RequestComplete {
          serial_id, image, ..
        } => {
          let decoded_image = image.read_image(&cam).try_decode().unwrap();
          let rgb_image = decoded_image
            .as_bgr()
            .unwrap()
            .as_rgb()
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

#[test]
#[should_panic]
fn panic_with_camera() {
  // Wait to ensure we can lock the camera (when running many tests back-to-back).
  std::thread::sleep(std::time::Duration::from_secs(5));
  let cm = CameraManager::new().unwrap();
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  cam.apply_config().unwrap();
  cam.start_stream().unwrap();
  panic!("Success");
}

#[test]
fn try_start_before_configure() {
  let cm = CameraManager::new().unwrap();
  println!("camers: {:?}", cm.get_camera_names());
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  assert!(matches!(
    cam.start_stream(),
    Err(LibcameraError::InvalidConfig)
  ));
  cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
}

#[test]
fn capture_then_drop() {
  // Wait to ensure we can lock the camera (when running many tests back-to-back).
  std::thread::sleep(std::time::Duration::from_secs(5));
  let cm = CameraManager::new().unwrap();
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  cam.apply_config().unwrap();
  cam.start_stream().unwrap();
  cam.capture_next_picture(0).unwrap();
  cam.capture_next_picture(0).unwrap();
}

#[test]
#[should_panic]
fn capture_then_panic() {
  // Wait to ensure we can lock the camera (when running many tests back-to-back).
  std::thread::sleep(std::time::Duration::from_secs(5));
  let cm = CameraManager::new().unwrap();
  let mut cam = cm.get_camera_by_name(&cm.get_camera_names()[0]).unwrap();
  cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  cam.apply_config().unwrap();
  cam.start_stream().unwrap();
  cam.capture_next_picture(0).unwrap();
  cam.capture_next_picture(0).unwrap();
  panic!("Success");
}

lazy_static::lazy_static! {
  static ref CAMERA_MANAGER: CameraManager = {
    let cm = CameraManager::new().unwrap();
    log::debug!("Camera Manager: {cm:?}");
    cm
  };
}

/// This test is designed to behave as similar as possible to ps-native.
#[test]
fn background_capture_thread() {
  use simplelog::SimpleLogger;
  use std::sync::mpsc::{self, channel};
  use std::sync::{Arc, RwLock};
  use std::thread;
  use std::time::Duration;

  use crate::controls;

  SimpleLogger::init(simplelog::LevelFilter::Trace, simplelog::Config::default()).unwrap();

  let mut cam = {
    let selected_cam = CAMERA_MANAGER.get_camera_names()[0].clone();
    log::debug!("Selected camera: {selected_cam:?}");
    CAMERA_MANAGER.get_camera_by_name(&selected_cam).unwrap()
  };
  log::debug!("Created camera.");
  let camera_config = cam.generate_config(&[StreamRole::Viewfinder]).unwrap();
  let stream_config = &mut camera_config.streams_mut()[0];
  log::debug!("Updating stream config...");
  stream_config.set_pixel_format(PixelFormat::Yuv420);
  stream_config.set_size(1280, 960);
  log::debug!("Applying config...");
  let status = cam.apply_config().unwrap();
  if status == ConfigStatus::Changed {
    log::debug!("Configuration updated by camera.");
  }
  log::debug!("Starting initial stream: {}", cam.start_stream().unwrap(),);

  log::info!("Camera ready!");
  let cam = Arc::new(RwLock::new(cam));
  let (req_tx, req_rx) = channel();
  let (frame_tx, frame_rx) = channel();
  let thread_join_handle = {
    let cam = cam.clone();
    thread::spawn(move || {
      for _i in 0..4 {
        cam.write().unwrap().capture_next_picture(0).unwrap();
      }
      let mut capture_next = false;
      let mut capture_id = None;
      loop {
        log::trace!("[CAM] Camera Loop");
        match req_rx.try_recv() {
          Ok(()) => capture_next = true,
          Err(mpsc::TryRecvError::Disconnected) => {
            log::debug!("Exiting Thread.");
            break;
          }
          Err(mpsc::TryRecvError::Empty) => {}
        };
        log::trace!("[CAM] Polling camera...");
        let mut cam = cam.write().unwrap();
        let events = cam.poll_events(None).unwrap();
        log::trace!("[CAM] Processing events...");
        for event in events {
          match event {
            CameraEvent::RequestComplete {
              serial_id, image, ..
            } => {
              if capture_id == Some(serial_id) {
                log::debug!("[CAM] Captured frame: {serial_id}");
                capture_id = None;
                frame_tx.send(image.read_image(&cam)).unwrap();
                log::debug!("[CAM] Sent.");
              }
              log::trace!("[CAM] Captured image, queuing another request.");
              let next = cam.capture_next_picture(0).unwrap();
              log::trace!("[CAM] Queued.");
              if capture_id.is_none() && capture_next {
                capture_id = Some(next);
                capture_next = false;
                log::debug!("[CAM] Capture next frame: {next}");
              }
            }
          }
        }
        log::trace!("[CAM] Done processing events, sleeping...");
        thread::sleep(Duration::from_millis(50));
      }
    })
  };
  log::debug!("Setting default controls.");
  {
    let mut cam = cam.write().unwrap();
    let controls = cam.get_controls_mut();
    if let Some(brightness) = controls.brightness.as_mut() {
      brightness.set_value_clamped(0.0);
    }
    if let Some(contrast) = controls.contrast.as_mut() {
      contrast.set_value_clamped(1.1);
    }
    if let Some(awb_enable) = controls.awb_enable.as_mut() {
      awb_enable.set_value_clamped(false);
    }
    if let Some(ae_enable) = controls.ae_enable.as_mut() {
      ae_enable.set_value_clamped(false);
    }
    if let Some(colour_gains) = controls.colour_gains.as_mut() {
      colour_gains.set_value_clamped(controls::FloatPair(1.63, 1.63));
    }
    if let Some(exposure_time) = controls.exposure_time.as_mut() {
      exposure_time.set_value_clamped(243);
    }
  }
  thread::sleep(Duration::from_secs(1));
  log::info!("Starting image capture...");
  for i in 0..50 {
    if (i % 4 > 1 || i > 40) && (i % 3 < 2 || i > 45) {
      log::debug!("Setting controls...");
      let mut cam = cam.write().unwrap();
      let controls = cam.get_controls_mut();
      if let Some(colour_gains) = controls.colour_gains.as_mut() {
        colour_gains.set_value_clamped(if i % 2 == 0 {
          controls::FloatPair(1.63, 1.63)
        } else {
          controls::FloatPair(1.0, 1.0)
        });
      }
      if let Some(exposure_time) = controls.exposure_time.as_mut() {
        exposure_time.set_value_clamped(243 + 10 - i * 2);
      }
      log::debug!("Controls set.");
    }
    thread::sleep(Duration::from_millis(if i == 35 {
      2000
    } else if i < 10 {
      200
    } else {
      0
    }));
    let start = std::time::Instant::now();
    log::debug!("[IMG] Requesting image...");
    req_tx.send(()).unwrap();
    log::debug!("[IMG] Waiting to get image...");
    let image = frame_rx.recv().unwrap();
    log::debug!("[IMG] Took picture in {:?}", start.elapsed());
    let image = image.try_decode().unwrap();
    log::debug!("[IMG] Decoded image in {:?}", start.elapsed());
    let image = image.as_bgr().unwrap();
    log::debug!("[IMG] Converted colour space in {:?}.", start.elapsed());
    let (width, height) = image.get_size();
    log::debug!("[IMG] Image size: {width}x{height}");
    std::fs::write("tmp.bgr", image.get_planes()[0]).unwrap();
  }
  log::debug!("[IMG] Dropping request tx.");
  drop(req_tx);
  log::debug!("[IMG] Joining thread.");
  thread_join_handle.join().unwrap();
  log::info!("[IMG] Exiting");
}
