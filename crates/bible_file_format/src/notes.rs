use uuid::Uuid;

use crate::{JsonSerde, TextRange, WordIndex};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BibleAnnotationsSave
{
    pub name: String,
    pub bible_name: String,
    pub description: String,
    pub notes: Vec<AnnotationSave>,
    pub highlighters: Vec<Highlighter>,
}

impl JsonSerde for BibleAnnotationsSave {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnnotationSave 
{
    pub annotation_type: AnnotationType,
    pub id: Uuid
}

impl AnnotationSave
{
    pub fn has_word(&self, word: WordIndex) -> bool
    {
        self.annotation_type.has_word(word)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AnnotationType
{
    Note
    {
        text: String,
        range: TextRange
    },
    CrossRef
    {
        text: String,
        a: TextRange,
        b: TextRange,
    },
    Highlight 
    {
        highlight_id: Uuid,
        range: TextRange,
    }
}

impl AnnotationType 
{
    pub fn has_word(&self, word: WordIndex) -> bool 
    {
        match self 
        {
            AnnotationType::Note { range, .. } => range.has_word(word),
            AnnotationType::CrossRef { a, b, .. } => a.has_word(word) || b.has_word(word),
            AnnotationType::Highlight { range, .. } => range.has_word(word),
        }
    }    
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct HighlightColor
{
    pub r: f32,
    pub b: f32,
    pub g: f32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Highlighter
{
    pub name: String,
    pub text: String,
    pub color: HighlightColor,
    pub id: Uuid,
}