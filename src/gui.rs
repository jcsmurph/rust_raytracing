use crate::imagegenerator::ImageGenerator;

#[derive(Default)]
pub struct MyEguiApp {
    height: f32,
    width: f32,
    rays_per_pixel: f32,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            height: 0.0,
            width: 0.0,
            rays_per_pixel: 0.0,
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("RayTracing Image Generation!");

           ui.add(egui::Slider::new(&mut self.width, 0.0..=1200.0).text("Adjust Image Width"));
           ui.add(egui::Slider::new(&mut self.height, 0.0..=800.0).text("Adjust Image Height"));
           ui.add(egui::Slider::new(&mut self.rays_per_pixel, 0.0..=100.0).text("Adjust Image Rays per Pixel"));

           if ui.button("Generate Image").clicked() {
                let image = ImageGenerator::new(self.height, self.width, self.rays_per_pixel);

                ImageGenerator::generate_image(&image);
           }
       });
   }
}
