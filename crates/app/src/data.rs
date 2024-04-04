use std::collections::HashMap;

use bible_file_format::{notes::{AnnotationSave, AnnotationType, BibleAnnotationsSave}, TextRange, Uuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChapterIndex 
{
    pub book_index: u8,
    pub chapter_index: u8,
}

impl ChapterIndex
{
    pub fn from_range(range: TextRange) -> Self
    {
        Self 
        {
            book_index: range.book_index,
            chapter_index: range.chapter_index
        }
    }

    pub fn has_range(&self, range: TextRange) -> bool 
    {
        range.book_index == self.book_index && 
        range.chapter_index == self.chapter_index
    }

    pub fn has_annotation(&self, annotation: &AnnotationSave) -> bool
    {
        match annotation.annotation_type
        {
            AnnotationType::Note { range, .. } => self.has_range(range),
            AnnotationType::CrossRef { a, b, .. } => self.has_range(a) || self.has_range(b),
            AnnotationType::Highlight { range, .. } => self.has_range(range),
        }
    }
}

pub struct AnnotationMap
{
    data: HashMap<ChapterIndex, ChapterAnnotations>
}

impl AnnotationMap
{
    pub fn new(save: &BibleAnnotationsSave) -> Self 
    {
        let mut data = HashMap::new();
        for annotation in &save.notes
        {
            let annotation = annotation.clone();
            match annotation.annotation_type
            {
                AnnotationType::Note { range, .. } => 
                {
                    get_or_insert_annotation(&mut data, annotation, ChapterIndex::from_range(range));
                },
                AnnotationType::CrossRef { a, b, .. } => 
                {
                    get_or_insert_annotation(&mut data, annotation.clone(), ChapterIndex::from_range(a));
                    get_or_insert_annotation(&mut data, annotation, ChapterIndex::from_range(b));
                },
                AnnotationType::Highlight { range, .. } => 
                {
                    get_or_insert_annotation(&mut data, annotation, ChapterIndex::from_range(range));
                },
            }
        }

        Self 
        {
            data
        }
    }

    pub fn get_chapter_annotations(&self, chapter_index: ChapterIndex) -> Option<&ChapterAnnotations>
    {
        self.data.get(&chapter_index)
    }
}

pub struct ChapterAnnotations
{
    map: HashMap<Uuid, AnnotationSave>
}

impl ChapterAnnotations
{
    pub fn new() -> Self
    {
        Self 
        {
            map: HashMap::new()
        }
    }

    pub fn add_annotation(&mut self, note: AnnotationSave)
    {
        self.map.insert(note.id, note);
    }

    pub fn get_all(&self) -> Vec<&AnnotationSave>
    {
        self.map.values().collect()
    }
}

fn get_or_insert_annotation(data: &mut HashMap<ChapterIndex, ChapterAnnotations>, note: AnnotationSave, chapter_index: ChapterIndex)
{
    if let Some(c) = data.get_mut(&chapter_index)
    {
        c.add_annotation(note);
    }
    else 
    {
        let mut c = ChapterAnnotations::new();   
        c.add_annotation(note);
        data.insert(chapter_index, c); 
    }
}