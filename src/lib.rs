#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

mod bridge;
/// Handles interfacing with cameras, get started using a CameraManager.
pub mod camera;
/// Camera and stream configuration, e.g. how many streams, resolutions, and pixel formats.
pub mod config;
/// Camera (runtime) controls, e.g. brightness, contrast.
pub mod controls;
/// Errors
pub mod error;
/// Decoding images from the camera
pub mod image;
/// All the things you *should* need to get started.
pub mod prelude;
#[cfg(test)]
mod test;

pub use error::{LibcameraError, Result};
