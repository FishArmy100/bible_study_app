pub mod gui;
pub mod data;
use std::sync::Arc;

use bible_file_format::{bible::BibleSave, notes::{AnnotationSave, AnnotationType, BibleAnnotationsSave}, JsonSerde, TextRange, Uuid};
use data::ChapterIndex;
use gui::{chapter_ui::VerseUi, BiblePanel};
use eframe::egui::{self, Grid, Ui};

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
        let annotations = Arc::new(get_test_annotations());

        Self 
        {
            bible: bible.clone(),
            panel: BiblePanel::new(bible, annotations),
        }
    }
}

impl eframe::App for MyEguiApp 
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) 
    {
        egui::CentralPanel::default().show(ctx, |ui| {
           self.panel.ui(ui);
           // test_ui(ui);
        });
    }
}

fn get_test_annotations() -> BibleAnnotationsSave
{
    let book_index = 18;
        let chapter_index = 117; // psalm 118
        let verse_index = 23; // verse 24
        let word_index = 1; // "is"

        let notes = vec![AnnotationSave {
            id: Uuid::new_v4(),
            annotation_type: AnnotationType::Note { 
                text: String::from("My test message"), 
                range: TextRange::word(book_index, chapter_index, verse_index as u16, word_index) 
            }
        }];

    BibleAnnotationsSave
    {
        name: "Test Save".into(),
        bible_name: "kjv".into(),
        description: "This is a test".into(),
        notes,
        highlighters: vec![]
    }
}
