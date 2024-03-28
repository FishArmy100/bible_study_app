pub mod gui;
use std::sync::Arc;

use bible_file_format::{bible::BibleSave, JsonSerde};
use gui::BiblePanel;
use eframe::egui;

fn main() -> Result<(), eframe::Error> 
{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))))
}

struct MyEguiApp 
{
    bible: Arc<BibleSave>,
    panel: BiblePanel,
}

impl MyEguiApp 
{
    fn new(cc: &eframe::CreationContext<'_>) -> Self 
    {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        cc.egui_ctx.set_pixels_per_point(2.0);
        let visuals = egui::Visuals::dark();
        cc.egui_ctx.set_visuals(visuals);

        let data = include_bytes!("../../../assets/test_bible.cjb");
        let bible = Arc::new(BibleSave::from_compressed_json(data).unwrap());
        Self 
        {
            bible: bible.clone(),
            panel: BiblePanel::new(bible),
        }
    }
}

impl eframe::App for MyEguiApp 
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ctx, |ui| {
           self.panel.ui(ui);
        });
    }
}
