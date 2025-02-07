use eframe::egui;
use eframe::egui::load::SizedTexture;
use eframe::egui::{Color32, Pos2, Stroke, StrokeKind};
use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

pub struct App {
    dynamic_image: DynamicImage,
    image_texture: egui::TextureHandle,
    factor: f32,
    is_drawing: bool,
    start_pos: Option<Pos2>,
    end_pos: Option<Pos2>,
    save_path: PathBuf,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, image: DynamicImage, factor: f32, save_path: String) -> Self {
        Self {
            dynamic_image: image.clone(),
            image_texture: load_dynamic_image(image, &cc.egui_ctx),
            factor,
            is_drawing: false,
            start_pos: None,
            end_pos: None,
            save_path: save_path.into(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let image_source = egui::ImageSource::Texture(SizedTexture::from_handle(&self.image_texture));
                ui.add(egui::Image::new(image_source).max_width(ui.available_width()));

                if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                    ui.label(format!("Mouse position: {:?}", mouse_pos));
                    let available_rect = ui.max_rect();
                    let painter = ui.painter();
                    // draw a cross-hair at the mouse position
                    painter.line_segment(
                        [
                            Pos2::new(available_rect.min.x, mouse_pos.y),
                            Pos2::new(available_rect.max.x, mouse_pos.y),
                        ],
                        Stroke::new(1.0, Color32::GRAY),
                    );
                    painter.line_segment(
                        [
                            Pos2::new(mouse_pos.x, available_rect.min.y),
                            Pos2::new(mouse_pos.x, available_rect.max.y),
                        ],
                        Stroke::new(1.0, Color32::GRAY),
                    );
                    // Check for mouse input
                    let input = ctx.input(|i| i.clone());
                    if input.pointer.any_pressed() {
                        if input.pointer.primary_down() {
                            // Start drawing if it's the first press
                            if self.start_pos.is_none() {
                                self.start_pos = Some(mouse_pos);
                                self.is_drawing = true;
                            }
                        }
                    } else if input.pointer.any_released() {
                        // Stop drawing when mouse button is released
                        // self.is_drawing = false;
                        // self.start_pos = None;
                        self.end_pos = Some(mouse_pos);
                        let sx = f32::min(self.start_pos.unwrap().x, self.end_pos.unwrap().x) * self.factor;
                        let sy = f32::min(self.start_pos.unwrap().y, self.end_pos.unwrap().y) * self.factor;
                        let width = (self.start_pos.unwrap().x - self.end_pos.unwrap().x).abs() * self.factor;
                        let height = (self.end_pos.unwrap().y - self.start_pos.unwrap().y).abs() * self.factor;

                        dbg!(sx, sy, width, height);
                        dbg!(self.dynamic_image.dimensions());

                        self.dynamic_image
                            .crop(sx as u32, sy as u32, width as u32, height as u32)
                            .save(self.save_path.clone())
                            .unwrap_or_else(|e| println!("Failed to save image: {:?}", e));

                        // Close the window
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    // Draw the rectangle if currently drawing
                    if let Some(start) = self.start_pos {
                        let rect = egui::Rect::from_two_pos(start, mouse_pos);
                        painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::GOLD), StrokeKind::Outside);
                    }
                } else {
                    ui.label("Mouse is not over the UI.");
                }
            });
    }
}

fn load_dynamic_image(image: DynamicImage, ctx: &egui::Context) -> egui::TextureHandle {
    // Convert the image to RGBA8 format
    let rgba_image = image.to_rgba8();

    // Get the image dimensions
    let dimensions = (rgba_image.width() as usize, rgba_image.height() as usize);

    // Convert the image to raw pixel data
    let pixels = rgba_image.into_raw();

    // Create an egui-compatible texture
    let texture = egui::ColorImage::from_rgba_unmultiplied(dimensions.into(), &pixels);
    ctx.load_texture("my_image", texture, egui::TextureOptions::default())
}
