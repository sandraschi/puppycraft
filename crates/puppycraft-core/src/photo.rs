use crate::error::PuppyCraftError;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoEditParams {
    /// 0, 90, 180, 270
    pub rotate_deg: u16,
    /// -50 .. +50
    pub brightness: i32,
    /// -50.0 .. +50.0
    pub contrast: f32,
    /// -30 .. +30 (warm golden doggie glow)
    pub warmth: i32,
    /// Optional normalized square/center crop (0.5 .. 1.0)
    pub crop_scale: f32,
    /// Add a clean Polaroid-style white frame around the photo
    pub polaroid_frame: bool,
}

impl Default for PhotoEditParams {
    fn default() -> Self {
        Self {
            rotate_deg: 0,
            brightness: 0,
            contrast: 0.0,
            warmth: 0,
            crop_scale: 1.0,
            polaroid_frame: false,
        }
    }
}

pub fn load_image(path: &Path) -> Result<DynamicImage, PuppyCraftError> {
    image::open(path).map_err(|e| PuppyCraftError::Image(format!("open {}: {e}", path.display())))
}

pub fn apply_edits(img: &DynamicImage, params: &PhotoEditParams) -> DynamicImage {
    let mut out = match params.rotate_deg % 360 {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img.clone(),
    };

    if params.crop_scale < 0.99 {
        let scale = params.crop_scale.clamp(0.3, 1.0);
        let (w, h) = out.dimensions();
        let nw = ((w as f32) * scale).round().max(16.0) as u32;
        let nh = ((h as f32) * scale).round().max(16.0) as u32;
        let x = (w.saturating_sub(nw)) / 2;
        let y = (h.saturating_sub(nh)) / 2;
        out = out.crop_imm(x, y, nw, nh);
    }

    if params.brightness != 0 {
        out = out.brighten(params.brightness.clamp(-80, 80));
    }

    if params.contrast.abs() > 0.5 {
        out = out.adjust_contrast(params.contrast.clamp(-60.0, 60.0));
    }

    if params.warmth != 0 {
        let warm = params.warmth.clamp(-40, 40);
        let rgba = out.to_rgba8();
        let (w, h) = rgba.dimensions();
        let mut buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
        for (x, y, px) in rgba.enumerate_pixels() {
            let r = (i32::from(px[0]) + warm).clamp(0, 255) as u8;
            let g = (i32::from(px[1]) + (warm / 3)).clamp(0, 255) as u8;
            let b = (i32::from(px[2]) - warm).clamp(0, 255) as u8;
            buf.put_pixel(x, y, Rgba([r, g, b, px[3]]));
        }
        out = DynamicImage::ImageRgba8(buf);
    }

    if params.polaroid_frame {
        let rgba = out.to_rgba8();
        let (w, h) = rgba.dimensions();
        let border = (w.min(h) / 18).max(12);
        let bottom = border * 3;
        let nw = w + border * 2;
        let nh = h + border + bottom;
        let mut framed: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_pixel(nw, nh, Rgba([248, 246, 240, 255]));
        for (x, y, px) in rgba.enumerate_pixels() {
            framed.put_pixel(x + border, y + border, *px);
        }
        out = DynamicImage::ImageRgba8(framed);
    }

    out
}

pub fn save_edited_image(
    img: &DynamicImage,
    params: &PhotoEditParams,
    dest: &Path,
) -> Result<(), PuppyCraftError> {
    let edited = apply_edits(img, params);
    edited
        .save(dest)
        .map_err(|e| PuppyCraftError::Image(format!("save {}: {e}", dest.display())))
}
