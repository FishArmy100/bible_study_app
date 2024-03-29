use bible_file_format::{bible::ChapterSave, notes::AnnotationSave, Uuid, WordIndex};
use eframe::egui::{Grid, Label, RichText, ScrollArea, Sense, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChapterIndex 
{
    pub book_index: u8,
    pub chapter_index: u8,
}

pub struct ChapterUi 
{
    header: RichText,
    verses: Vec<VerseUi>
}

impl ChapterUi
{
    pub fn new(chapter: &ChapterSave, chapter_index: ChapterIndex, book_name: &str, annotations: &[AnnotationSave]) -> Self 
    {
        let header = RichText::new(format!("{} Chapter {}:", book_name, chapter_index.chapter_index + 1)).strong().heading();

        let mut verses = vec![];
        for i in 0..chapter.verses.len()
        {
            let text = &chapter.verses[i].text;
            let verse_ui = VerseUi::new(text, i as u32, chapter_index, annotations);
            verses.push(verse_ui);
        }
        
        Self 
        {
            header,
            verses,
        }
    }

    pub fn ui(&self, ui: &mut Ui)
    {   
        ScrollArea::vertical().id_source("verses_area").show(ui, |ui| {
            ui.add(Label::new(self.header.clone()));
            for verse_ui in &self.verses
            {
                verse_ui.ui(ui);
            }
        });
    }
}

pub struct VerseUi
{
    number_text: RichText,
    word_uis: Vec<WordUi>,
}

impl VerseUi
{
    pub fn new(text: &str, number_index: u32, chapter: ChapterIndex, annotations: &[AnnotationSave]) -> Self 
    {
        let number_text = RichText::new((number_index + 1).to_string()).strong();
        let words = text.split(char::is_whitespace);

        let mut word_uis = vec![];
        let mut i = 0;
        for word in words
        {
            let word_index = WordIndex {
                book_index: chapter.book_index,
                chapter_index: chapter.chapter_index,
                verse_index: number_index as u16,
                word_index: i
            };

            let mut note_ids = vec![];

            for a in annotations
            {
                if a.has_word(word_index)
                {
                    note_ids.push(a.id)
                }
            }

            let word_ui = WordUi::new(word, note_ids, word_uis.last());

            word_uis.push(word_ui);

            i += 1;
        }

        Self 
        {
            number_text,
            word_uis,
        }
    }

    pub fn ui(&self, ui: &mut Ui)
    {
        Grid::new(self.number_text.text()).num_columns(2).show(ui, |ui| {
            ui.label(self.number_text.clone());
            ui.horizontal_wrapped(|ui| {
                let spacing_old = ui.spacing().item_spacing;
                ui.spacing_mut().item_spacing = (0.0, 0.0).into();
                for word in &self.word_uis
                {
                    word.ui(ui);
                }

                ui.spacing_mut().item_spacing = spacing_old;
            })
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpacerMode
{
    None,
    Some,
    Underlined
}

pub struct WordUi
{
    text: RichText,
    note_ids: Vec<Uuid>,
    spacer_mode: SpacerMode,
}

impl WordUi
{
    pub fn new(word: &str, note_ids: Vec<Uuid>, previous: Option<&WordUi>) -> Self
    {
        let mut text = RichText::new(word);
        if note_ids.len() > 0 { text = text.underline() };

        let spacer_mode = match previous {
            Some(p) => if p.note_ids.len() > 0 { SpacerMode::Underlined } 
                       else { SpacerMode::Some },
            None => SpacerMode::None,
        };

        Self
        {
            text,
            note_ids,
            spacer_mode
        }
    }

    pub fn ui(&self, ui: &mut Ui)
    {
        match self.spacer_mode
        {
            SpacerMode::None => {},
            SpacerMode::Some => { ui.add(spacer_label(false)); },
            SpacerMode::Underlined => { ui.add(spacer_label(false)); },
        }

        ui.add(Label::new(self.text.clone()).sense(Sense::click()));
    }
}

pub fn spacer_label(underlined: bool) -> Label
{
    let mut text = RichText::new(" ");
    if underlined { text = text.underline() };
    Label::new(text)
}