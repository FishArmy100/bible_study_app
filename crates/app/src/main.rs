pub mod parsing;
pub mod gui;

use std::sync::Arc;

use bible_file_format::{Bible, CompressionLevel, JsonMode};
use gui::BiblePanel;
use eframe::egui;
use parsing::bible_from_md;

fn main() -> Result<(), eframe::Error> 
{
    let bible = bible_from_md(include_str!("../../../assets/test_genesis_kjv.md")).unwrap();
    let low_data = bible.to_compressed_json(CompressionLevel::Low).unwrap();
    let high_data = bible.to_compressed_json(CompressionLevel::High).unwrap();
    
    println!("low: {}; high: {}", low_data.len(), high_data.len());

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))))
}

struct MyEguiApp 
{
    bible: Arc<Bible>,
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

        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let bible = Arc::new(bible_from_md(include_str!("../../../assets/genesis_kjv.md")).unwrap());
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
           self.panel.ui(ui)
        });
    }
}
