pub mod parsing;

use markdown::mdast::{List, Node};

#[derive(Debug)]
pub struct Bible
{
    pub books: Vec<Book>
}

#[derive(Debug)]
pub struct Book
{
    pub id: u32,
    pub name: String,
    pub chapters: Vec<Chapter>
}

#[derive(Debug)]
pub struct Chapter
{
    pub number: u32,
    pub verses: Vec<Verse>
}

#[derive(Debug)]
pub struct Verse 
{
    pub number: u32,
    pub text: String,
}