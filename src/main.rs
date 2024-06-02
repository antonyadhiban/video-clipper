extern crate eframe;
extern crate egui_extras;
extern crate image;
use eframe::egui;
use egui_extras::RetainedImage;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgba};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    image2: RetainedImage,
    image3: Option<RetainedImage>,
    image: Option<RetainedImage>,
    buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        // Load image2 from mono.png and convert to RetainedImage
        let image2 = ImageReader::open("mono.png")
            .expect("Failed to open image")
            .decode()
            .expect("Failed to decode image");
        let rgba_image = image2.to_rgba8();
        let max_dimension = 512;
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let (new_width, new_height) = if size[0] > size[1] {
            (
                max_dimension,
                (size[1] as f32 * (max_dimension as f32 / size[0] as f32)) as u32,
            )
        } else {
            (
                (size[0] as f32 * (max_dimension as f32 / size[1] as f32)) as u32,
                max_dimension,
            )
        };

        let resized_image2 =
            image2.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
        let rgba_image = resized_image2.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied(size, rgba_image.as_flat_samples().as_slice());
        let retained_image2 = RetainedImage::from_color_image("image2", color_image);

        Self {
            image2: retained_image2,
            image3: None,
            image: None,
            buffer: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            self.image2.show(ui);

            if let Some(img) = &self.image3 {
                img.show(ui);
            }

            if ui.button("Open Image").clicked() {
                if let Some(file_path) = rfd::FileDialog::new().pick_file() {
                    if let Ok(image3) = ImageReader::open(file_path)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                        .and_then(|r| {
                            r.decode()
                                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                        })
                    {
                        let rgba_image = image3.to_rgba8();
                        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                        let max_dimension = 512;
                        let (new_width, new_height) = if size[0] > size[1] {
                            (
                                max_dimension,
                                (size[1] as f32 * (max_dimension as f32 / size[0] as f32)) as u32,
                            )
                        } else {
                            (
                                (size[0] as f32 * (max_dimension as f32 / size[1] as f32)) as u32,
                                max_dimension,
                            )
                        };
                        let resized_image3 =
                            image3.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
                        let rgba_image3 = resized_image3.to_rgba8();
                        let size = [rgba_image3.width() as usize, rgba_image3.height() as usize];
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            rgba_image3.as_flat_samples().as_slice(),
                        );
                        let retained_image3 =
                            RetainedImage::from_color_image("image3", color_image);
                        self.image3 = Some(retained_image3);
                    }
                }
            }

            if let Some(img) = &self.image {
                img.show(ui);
                if ui.button("save").clicked() {
                    if let Some(buf) = &self.buffer {
                        println!("img saved!");
                        buf.save("test.png").unwrap();
                    }
                }
            }
            if ui.button("Render").clicked() {
                let w = 200;
                let h = 100;
                self.buffer = Some(image::ImageBuffer::new(w, h));
                for (x, y, pixels) in self.buffer.as_mut().unwrap().enumerate_pixels_mut() {
                    let a = 1.0_f32;
                    *pixels = image::Rgba([
                        (x as f32 / w as f32 * 255.0_f32) as u8,
                        (y as f32 / h as f32 * 255.0_f32) as u8,
                        0.2_f32 as u8,
                        (a * 255.0).min(255.0).max(0.0) as u8,
                    ]);
                }

                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [w as usize, h as usize],
                    &self.buffer.as_ref().unwrap(),
                );
                let render_result = RetainedImage::from_color_image("0.png", color_image);
                self.image = Some(render_result);
            }
        });
    }
}
