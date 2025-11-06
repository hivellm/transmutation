use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use ndarray::Array4;

/// Image preprocessing for ML models
///
/// Based on docling's layout_model.py preprocessing
use crate::error::Result;

/// ImageNet normalization parameters (used by most vision models)
pub const IMAGENET_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
pub const IMAGENET_STD: [f32; 3] = [0.229, 0.224, 0.225];

/// Standard input size for layout model
pub const LAYOUT_MODEL_SIZE: u32 = 1025;

/// Preprocess image for layout model inference
///
/// Steps:
/// 1. Resize to 1025x1025 (maintaining aspect ratio with padding)
/// 2. Normalize with ImageNet mean/std
/// 3. Convert to NCHW tensor format (1, 3, H, W)
///
/// Reference: transmutation/docling/docling/models/layout_model.py
pub fn preprocess_for_layout(image: &DynamicImage) -> Result<Array4<f32>> {
    // Resize to model input size
    let resized = resize_with_padding(image, LAYOUT_MODEL_SIZE, LAYOUT_MODEL_SIZE)?;

    // Convert to RGB if needed
    let rgb_image = resized.to_rgb8();

    // Convert to ndarray and normalize
    let tensor = image_to_tensor(&rgb_image, &IMAGENET_MEAN, &IMAGENET_STD)?;

    Ok(tensor)
}

/// Resize image to target size with padding to maintain aspect ratio
fn resize_with_padding(
    image: &DynamicImage,
    target_width: u32,
    target_height: u32,
) -> Result<DynamicImage> {
    let (width, height) = image.dimensions();
    let aspect_ratio = width as f32 / height as f32;
    let target_aspect_ratio = target_width as f32 / target_height as f32;

    let (new_width, new_height) = if aspect_ratio > target_aspect_ratio {
        // Width is limiting factor
        (target_width, (target_width as f32 / aspect_ratio) as u32)
    } else {
        // Height is limiting factor
        ((target_height as f32 * aspect_ratio) as u32, target_height)
    };

    // Resize image
    let resized = image.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);

    // Create canvas with padding
    let mut canvas = DynamicImage::new_rgb8(target_width, target_height);

    // Center the image
    let x_offset = (target_width - new_width) / 2;
    let y_offset = (target_height - new_height) / 2;

    image::imageops::overlay(&mut canvas, &resized, x_offset as i64, y_offset as i64);

    Ok(canvas)
}

/// Convert RGB image to normalized tensor (1, 3, H, W)
fn image_to_tensor(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    mean: &[f32; 3],
    std: &[f32; 3],
) -> Result<Array4<f32>> {
    let (width, height) = image.dimensions();
    let mut tensor = Array4::<f32>::zeros((1, 3, height as usize, width as usize));

    // Convert from HWC to CHW and normalize
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            for c in 0..3 {
                let value = pixel[c] as f32 / 255.0; // [0, 1]
                let normalized = (value - mean[c]) / std[c];
                tensor[[0, c, y as usize, x as usize]] = normalized;
            }
        }
    }

    Ok(tensor)
}

/// Preprocess image for table structure model
///
/// Table model uses 2x upscaling (144 DPI) for better precision
pub fn preprocess_for_table(image: &DynamicImage, scale: f32) -> Result<Array4<f32>> {
    let (width, height) = image.dimensions();
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;

    let resized = image.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
    let rgb_image = resized.to_rgb8();

    let tensor = image_to_tensor(&rgb_image, &IMAGENET_MEAN, &IMAGENET_STD)?;

    Ok(tensor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_with_padding() {
        let image = DynamicImage::new_rgb8(800, 600);
        let resized = resize_with_padding(&image, 1025, 1025).unwrap();

        assert_eq!(resized.dimensions(), (1025, 1025));
    }

    #[test]
    fn test_tensor_shape() {
        let image = DynamicImage::new_rgb8(1025, 1025);
        let tensor = preprocess_for_layout(&image).unwrap();

        assert_eq!(tensor.shape(), &[1, 3, 1025, 1025]);
    }

    #[test]
    fn test_normalization_range() {
        let image = DynamicImage::new_rgb8(100, 100);
        let tensor = preprocess_for_layout(&image).unwrap();

        // Values should be roughly in range [-3, 3] after normalization
        let min = tensor.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = tensor.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        assert!(min > -5.0 && min < 0.0);
        assert!(max > 0.0 && max < 5.0);
    }
}
