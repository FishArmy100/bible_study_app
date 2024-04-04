pub mod chapter_ui;
pub mod note_ui;

use std::sync::Arc;
use eframe::egui::{self, Button, Label, ScrollArea, Sense};

use bible_file_format::{bible::{BibleSave, ChapterSave}, notes::{AnnotationSave, BibleAnnotationsSave}};

use crate::data::ChapterIndex;

use self::chapter_ui::ChapterUi;

pub struct BiblePanel
{
    bible: Arc<BibleSave>,
    chapter_index: ChapterIndex, 
    chapter_ui: ChapterUi,
    annotations: Arc<BibleAnnotationsSave>
}

impl BiblePanel
{
    pub fn new(bible: Arc<BibleSave>, annotations: Arc<BibleAnnotationsSave>) -> Self 
    {
        let index = ChapterIndex {
            book_index: 0,
            chapter_index: 0,
        };

        let chapter_ui = get_chapter_ui(&bible, index, &annotations.notes);

        Self 
        {
            bible,
            chapter_index: index,
            chapter_ui,
            annotations,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui)
    {
        egui::Grid::new("selector").show(ui, |ui| {
            let old_chapter = self.chapter_index;
            let old_book = old_chapter.book_index;

            let book_label = &self.bible.books[self.chapter_index.book_index as usize].name;
            egui::ComboBox::from_id_source("book_selector")
                .selected_text(book_label)
                .show_ui(ui, |ui| {
                    let mut index = 0;
                    for b in &self.bible.books
                    {
                        ui.selectable_value(&mut self.chapter_index.book_index, index, &b.name);
                        index += 1;
                    }
                });

            if old_book != self.chapter_index.book_index { self.chapter_index.chapter_index = 0 }
            
            let chapter_label = format!("Chapter {}", self.chapter_index.chapter_index + 1);
            egui::ComboBox::from_id_source("chapter_selector")
                .selected_text(chapter_label)
                .show_ui(ui, |ui| {
                    let chapter_count = self.bible.books[self.chapter_index.book_index as usize].chapters.len() as u32;
                    for i in 0..chapter_count
                    {
                        ui.selectable_value(&mut self.chapter_index.chapter_index, i as u8, &format!("Chapter {}", i + 1));
                    }
                });

            if old_chapter != self.chapter_index
            {
                self.chapter_ui = get_chapter_ui(&self.bible, self.chapter_index, &self.annotations.notes)
            }
        });

        ui.separator();
        
        self.chapter_ui.ui(ui);
    }
}

fn get_chapter_ui(bible: &BibleSave, index: ChapterIndex, annotations: &[AnnotationSave]) -> ChapterUi
{
    let book = &bible.books[index.book_index as usize];
    let chapter = &book.chapters[index.chapter_index as usize];
    ChapterUi::new(chapter, index, &book.name, annotations)
}