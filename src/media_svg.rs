//! SVG rasterization helpers for media derivatives

use image::{DynamicImage, RgbaImage};
use resvg::{render, tiny_skia, usvg};
use std::path::Path;

pub fn decode_svg(bytes: &[u8], path: &Path) -> Option<DynamicImage> {
    let mut options = usvg::Options {
        resources_dir: path.parent().map(|dir| dir.to_path_buf()),
        ..usvg::Options::default()
    };
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_data(bytes, &options).ok()?;
    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())?;
    render(
        &tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    RgbaImage::from_raw(size.width(), size.height(), pixmap.take()).map(DynamicImage::ImageRgba8)
}
