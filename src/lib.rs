use eframe::{egui, run_native};
use image::buffer::ConvertBuffer;
use image::DynamicImage;
use xcap::Monitor;

mod app;
mod tests;

use app::App;

pub fn get_screen_snip(save_path: String) {
    let viewport_builder = egui::ViewportBuilder::default()
        .with_title("Screenshot Viewport")
        .with_resizable(false)
        .with_always_on_top()
        .with_maximized(true)
        .with_decorations(false)
        .with_transparent(true);
    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    let monitors = Monitor::all().unwrap();
    let monitor = &monitors[0];
    dbg!(monitor.x(), monitor.y());
    let rgba_image = monitor.capture_image().unwrap();
    let rgb_image = rgba_image.convert();
    println!("Image size: {:?}", rgb_image.dimensions());
    let factor = monitor.scale_factor();
    println!("Scale factor: {:?}", factor);
    let image = DynamicImage::ImageRgba8(rgb_image);

    run_native(
        "App",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc, image, factor, save_path)))),
    )
    .unwrap();
}
