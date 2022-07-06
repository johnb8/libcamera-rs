use crate::{LibcameraError, Result};

/// Represents an image.
pub trait CameraImage<const PLANES: usize> {
  /// Create an image out of a size and the image data planes.
  fn from_planes(width: usize, height: usize, planes: [Vec<u8>; PLANES]) -> Option<Self>
  where
    Self: Sized;
  /// Convert this image into BGR format.
  fn as_bgr(&self) -> Option<BgrImage>;
  /// Get the size of this image.
  fn get_size(&self) -> (usize, usize);
  /// Get the raw pixel planes for this image.
  fn get_planes(&self) -> [&[u8]; PLANES];
}

/// Contains an image in any format.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MultiImage {
  Bgr(BgrImage),
  Rgb(RgbImage),
  Yuyv(YuyvImage),
  Yuv420(Yuv420Image),
}

impl MultiImage {
  /// Convert this image into a [`BgrImage`].
  pub fn as_bgr(&self) -> Option<BgrImage> {
    match self {
      MultiImage::Bgr(img) => img.as_bgr(),
      MultiImage::Rgb(img) => img.as_bgr(),
      MultiImage::Yuyv(img) => img.as_bgr(),
      MultiImage::Yuv420(img) => img.as_bgr(),
    }
  }
  /// Get the size of this image.
  pub fn get_size(&self) -> (usize, usize) {
    match self {
      MultiImage::Bgr(img) => img.get_size(),
      MultiImage::Rgb(img) => img.get_size(),
      MultiImage::Yuyv(img) => img.get_size(),
      MultiImage::Yuv420(img) => img.get_size(),
    }
  }
}

/// Contains an image in BGR Format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BgrImage {
  width: usize,
  height: usize,
  data: Vec<u8>,
}

impl CameraImage<1> for BgrImage {
  fn from_planes(width: usize, height: usize, planes: [Vec<u8>; 1]) -> Option<BgrImage> {
    let [data] = planes;
    if width * height * 3 == data.len() {
      Some(BgrImage {
        width,
        height,
        data,
      })
    } else {
      None
    }
  }
  fn as_bgr(&self) -> Option<BgrImage> {
    Some(self.clone())
  }
  fn get_size(&self) -> (usize, usize) {
    (self.width, self.height)
  }
  fn get_planes(&self) -> [&[u8]; 1] {
    [&self.data]
  }
}

impl BgrImage {
  pub fn as_rgb(&self) -> Option<RgbImage> {
    RgbImage::from_planes(
      self.width,
      self.height,
      [self
        .data
        .chunks(3)
        .flat_map(|chunk| {
          if let &[b, g, r] = chunk {
            [r, g, b]
          } else {
            panic!("Exact chunks not exact!");
          }
        })
        .collect()],
    )
  }
}

/// Contains an image in RGB Format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbImage {
  width: usize,
  height: usize,
  data: Vec<u8>,
}

impl CameraImage<1> for RgbImage {
  fn from_planes(width: usize, height: usize, planes: [Vec<u8>; 1]) -> Option<RgbImage> {
    let [data] = planes;
    if width * height * 3 == data.len() {
      Some(RgbImage {
        width,
        height,
        data,
      })
    } else {
      None
    }
  }
  fn as_bgr(&self) -> Option<BgrImage> {
    BgrImage::from_planes(
      self.width,
      self.height,
      [self
        .data
        .chunks(3)
        .flat_map(|chunk| {
          if let &[r, g, b] = chunk {
            [b, g, r]
          } else {
            panic!("Exact chunks not exact!");
          }
        })
        .collect()],
    )
  }
  fn get_size(&self) -> (usize, usize) {
    (self.width, self.height)
  }
  fn get_planes(&self) -> [&[u8]; 1] {
    [&self.data]
  }
}

/// These functions are only available with the `image` feature/crate.
#[cfg(feature = "image")]
impl RgbImage {
  /// Decode an [`RgbImage`] from a JPEG image stream in a Vec<u8>.
  ///
  /// Available only with the `image` feature/crate.
  pub fn decode_jpeg(data: &[u8]) -> Result<RgbImage> {
    let image = image::load_from_memory_with_format(data, image::ImageFormat::Jpeg)?;
    println!("Image loaded");
    if let image::DynamicImage::ImageRgb8(img) = image {
      let (width, height) = img.dimensions();
      println!("JPEG image size {width}x{height}.");
      Ok(
        RgbImage::from_planes(width as usize, height as usize, [img.into_raw()])
          .ok_or(LibcameraError::BadImageFormat)?,
      )
    } else {
      Err(LibcameraError::BadImageFormat)
    }
  }
  /// Encode an [`RgbImage`] to a PNG image stream in a Vec<u8>.
  ///
  /// Available only with the `image` feature/crate.
  pub fn encode_png(&self) -> Result<Vec<u8>> {
    let mut buffer = std::io::Cursor::new(Vec::new());
    image::write_buffer_with_format(
      &mut buffer,
      &self.data,
      self.width as u32,
      self.height as u32,
      image::ColorType::Rgb8,
      image::ImageOutputFormat::Png,
    )?;
    Ok(buffer.into_inner())
  }
}

fn yuv2rgb(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
  (
    (y as f32 + (1.370705 * (v as f32 - 128.0))).clamp(0.0, 255.0) as u8,
    (y as f32 - (0.698001 * (v as f32 - 128.0)) - (0.337633 * (u as f32 - 128.0))).clamp(0.0, 255.0)
      as u8,
    (y as f32 + (1.732446 * (u as f32 - 128.0))).clamp(0.0, 255.0) as u8,
  )
}

/// Contains an image in YUVU 4:2:2 Format.
#[derive(Debug, Clone)]
pub struct YuyvImage {
  width: usize,
  height: usize,
  data: Vec<u8>,
}

impl CameraImage<1> for YuyvImage {
  fn from_planes(width: usize, height: usize, planes: [Vec<u8>; 1]) -> Option<YuyvImage> {
    let [data] = planes;
    if width * height * 2 == data.len() {
      Some(YuyvImage {
        width,
        height,
        data,
      })
    } else {
      None
    }
  }
  fn as_bgr(&self) -> Option<BgrImage> {
    BgrImage::from_planes(
      self.width,
      self.height,
      [self
        .data
        .chunks_exact(4)
        .flat_map(|chunk| {
          if let &[y1, u, y2, v] = chunk {
            // Map each section of y*u*y*v* to two BGR pixels
            let (r1, g1, b1) = yuv2rgb(y1, u, v);
            let (r2, g2, b2) = yuv2rgb(y2, u, v);
            [b1, g1, r1, b2, g2, r2]
          } else {
            panic!("Exact chunks not exact!");
          }
        })
        .collect()],
    )
  }
  fn get_size(&self) -> (usize, usize) {
    (self.width, self.height)
  }
  fn get_planes(&self) -> [&[u8]; 1] {
    [&self.data]
  }
}

/// Contains an image in YUV 4:2:0 Format.
#[derive(Debug, Clone)]
pub struct Yuv420Image {
  width: usize,
  height: usize,
  y_plane: Vec<u8>,
  u_plane: Vec<u8>,
  v_plane: Vec<u8>,
}

impl CameraImage<3> for Yuv420Image {
  fn from_planes(width: usize, height: usize, planes: [Vec<u8>; 3]) -> Option<Yuv420Image> {
    let [y_plane, u_plane, v_plane] = planes;
    if width * height == y_plane.len()
      && width / 2 * height / 2 == u_plane.len()
      && width / 2 * height / 2 == v_plane.len()
    {
      Some(Yuv420Image {
        width,
        height,
        y_plane,
        u_plane,
        v_plane,
      })
    } else {
      None
    }
  }
  fn as_bgr(&self) -> Option<BgrImage> {
    BgrImage::from_planes(
      self.width,
      self.height,
      [self
        .y_plane
        .chunks_exact(self.width) // Get each line of y*
        .zip(
          // Zip it with each line of u* and v* (half width+height)
          self
            .u_plane // For the u plane...
            .chunks_exact(self.width / 2)
            .map(|line| line.iter().flat_map(|&u| [u, u])) // Double the width
            .zip(
              // Zip the u and v planes
              self
                .v_plane // For the v plane...
                .chunks_exact(self.width / 2)
                .map(|line| line.iter().flat_map(|&v| [v, v])), // Double the width
            )
            .flat_map(|line| [line.clone(), line]), // Double the height
        )
        .flat_map(|(y_line, (u_line, v_line))| {
          // Re-zip y*, u* and v* per-pixel instead of per-line.
          y_line
            .iter()
            .zip(u_line.into_iter().zip(v_line.into_iter()))
        })
        .flat_map(|(&y, (u, v))| {
          // Convert to RGB
          let (r, g, b) = yuv2rgb(y, u, v);
          [r, g, b]
        })
        .collect()],
    )
  }
  fn get_size(&self) -> (usize, usize) {
    (self.width, self.height)
  }
  fn get_planes(&self) -> [&[u8]; 3] {
    [&self.y_plane, &self.u_plane, &self.v_plane]
  }
}
