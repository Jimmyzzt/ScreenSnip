#[cfg(test)]
mod tests {
    use image::{DynamicImage, RgbImage};
    use xcap::Monitor;

    #[test]
    fn test_screen_snip() {
        crate::get_screen_snip("screenshot.png".to_string());
    }

    #[test]
    fn test_smth() {
        let monitors = Monitor::all().unwrap();
        let monitor = &monitors[0];
        let rgba_image = monitor.capture_image().unwrap();
        println!("Image size: {:?}", rgba_image.dimensions());
        let factor = monitor.scale_factor();
        println!("Scale factor: {:?}", factor);
    }
}
