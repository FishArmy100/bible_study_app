use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Word
{
    pub word: String,
    pub italicized: bool,
    pub red: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verse
{
    pub words: Vec<Word>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter
{
    pub verses: Vec<Verse>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book 
{
    pub name: String,
    pub desc: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bible
{
    pub name: String,
    pub books: Vec<Book>,
}