pub mod bible;

use std::sync::Arc;

use bible::view::BiblePanel;
use eframe::egui;

use crate::bible::Bible;

fn main() -> Result<(), eframe::Error> 
{

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

        let bible = Arc::new(Bible::from_md(include_str!("../assets/test_genesis_kjv.md")).unwrap());
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
           self.panel.update(ui)
        });
    }
}
