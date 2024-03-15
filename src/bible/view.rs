use std::sync::Arc;
use eframe::egui;

use super::Bible;

pub struct BiblePanel
{
    bible: Arc<Bible>,
    book: u32,
    chapter: u32,
}

impl BiblePanel
{
    pub fn new(bible: Arc<Bible>) -> Self 
    {
        Self 
        {
            bible,
            book: 0,
            chapter: 0,
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui)
    {
        egui::Grid::new("book_selector").show(ui, |ui| {
            let book_label = &self.bible.books[self.book as usize].name;
            egui::ComboBox::from_id_source("book_selector")
                .selected_text(book_label)
                .show_ui(ui, |ui| {
                    let mut index = 0;
                    for b in &self.bible.books
                    {
                        ui.selectable_value(&mut self.book, index, &b.name);
                        index += 1;
                    }
                });

            let chapter_number = &self.bible.books[self.book as usize].chapters[self.chapter as usize].number;
            let chapter_label = format!("Chapter {}", chapter_number);
            egui::ComboBox::from_id_source("chapter_selector")
                .selected_text(chapter_label)
                .show_ui(ui, |ui| {
                    let chapter_count = self.bible.books[self.book as usize].chapters.len() as u32;
                    for i in 0..chapter_count
                    {
                        ui.selectable_value(&mut self.chapter, i, &format!("Chapter {}", i + 1));
                    }
                })
        });

        ui.separator();
    }
}