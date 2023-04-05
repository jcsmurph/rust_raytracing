use crate::imagegenerator::ImageGenerator;
use eframe::egui;
use egui_extras::RetainedImage;

pub struct MyEguiApp {
    pub height: i32,
    pub width: i32,
    pub image: RetainedImage,
}

impl MyEguiApp {}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes("output.ppm", include_bytes!("output.ppm"))
                .unwrap(),
            height: 0,
            width: 0,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RayTracing Image Generation");

            ui.add(egui::Slider::new(&mut self.width, 200..=1200).text("Adjust Image Width"));
            ui.add(egui::Slider::new(&mut self.height, 100..=1200).text("Adjust Image Height"));

            if ui.button("Generate Image").clicked() {
                let image_generator = ImageGenerator::new(self.height.into(), self.width.into());

                ImageGenerator::generate_image(&image_generator);
            }


            self.image.show(ui);


        });
    }
}
