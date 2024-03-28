use crate::{JsonSerde, TextRange};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BibleAnnotationsSave
{
    pub name: String,
    pub bible_name: String,
    pub description: String,
    pub notes: Vec<AnnotationType>,
    pub highlighters: Vec<Highlighter>,
}

impl JsonSerde for BibleAnnotationsSave {}

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
        highlight_id: u16,
        range: TextRange,
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
}