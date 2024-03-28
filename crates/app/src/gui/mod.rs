use std::sync::Arc;
use eframe::egui::{self, Button, Label, ScrollArea, Sense};

use bible_file_format::bible::{BibleSave, ChapterSave};

pub struct BiblePanel
{
    bible: Arc<BibleSave>,
    book_index: u32,
    chapter_index: u32,
}

impl BiblePanel
{
    pub fn new(bible: Arc<BibleSave>) -> Self 
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
        egui::Grid::new("selector").show(ui, |ui| {
            let old = self.book_index;
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

            if old != self.book_index { self.chapter_index = 0 }
            
            let chapter_label = format!("Chapter {}", self.chapter_index + 1);
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
            let verses = &self.chapter().verses;
            for i in 0..verses.len()
            {
                let v = &verses[i];
                egui::Grid::new(i).max_col_width(ui.available_width() - 50.0).show(ui, |ui| {
                    
                    let verse_number = Label::new((i + 1).to_string()).selectable(false);
                    ui.add(verse_number);

                    let verse_text = Label::new(&v.text).wrap(true).sense(Sense::click());
                    ui.add(verse_text);
                });
            }
        });
    }

    fn chapter(&self) -> &ChapterSave
    {
        &self.bible.books[self.book_index as usize].chapters[self.chapter_index as usize]
    }
}