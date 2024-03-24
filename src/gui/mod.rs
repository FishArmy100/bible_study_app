use std::sync::Arc;
use eframe::egui::{self, Label, ScrollArea};

use crate::bible::{Bible, Chapter};

pub struct BiblePanel
{
    bible: Arc<Bible>,
    book_index: u32,
    chapter_index: u32,
}

impl BiblePanel
{
    pub fn new(bible: Arc<Bible>) -> Self 
    {
        Self 
        {
            bible,
            book_index: 0,
            chapter_index: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui)
    {
        egui::Grid::new("book_selector").show(ui, |ui| {
            let book_label = &self.bible.books[self.book_index as usize].name;
            egui::ComboBox::from_id_source("book_selector")
                .selected_text(book_label)
                .show_ui(ui, |ui| {
                    let mut index = 0;
                    for b in &self.bible.books
                    {
                        ui.selectable_value(&mut self.book_index, index, &b.name);
                        index += 1;
                    }
                });

            let chapter = self.chapter();
            let chapter_label = format!("Chapter {}", chapter.number);
            egui::ComboBox::from_id_source("chapter_selector")
                .selected_text(chapter_label)
                .show_ui(ui, |ui| {
                    let chapter_count = self.bible.books[self.book_index as usize].chapters.len() as u32;
                    for i in 0..chapter_count
                    {
                        ui.selectable_value(&mut self.chapter_index, i, &format!("Chapter {}", i + 1));
                    }
                })
        });

        ui.separator();

        ScrollArea::vertical().id_source("verses_area").show(ui, |ui| {
            for v in &self.chapter().verses
            {
                egui::Grid::new(v.number).max_col_width(ui.available_width() - 50.0).show(ui, |ui| {
                    ui.label(v.number.to_string());
                    let text = Label::new(&v.text).wrap(true);
                    ui.add(text);
                });
            }
        });
    }

    fn chapter(&self) -> &Chapter
    {
        &self.bible.books[self.book_index as usize].chapters[self.chapter_index as usize]
    }
}