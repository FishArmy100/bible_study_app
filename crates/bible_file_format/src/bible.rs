use crate::JsonSerde;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BibleSave
{
    pub name: String,
    pub description: Option<String>,
    pub copyright: Option<String>,
    pub books: Vec<BookSave>
}

impl JsonSerde for BibleSave {}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BookSave
{
    pub name: String,
    pub testament: String,
    pub chapters: Vec<ChapterSave>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ChapterSave
{
    pub verses: Vec<VerseSave>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VerseSave 
{
    pub text: String,
}
