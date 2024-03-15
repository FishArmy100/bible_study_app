pub mod view;

use markdown::mdast::{List, Node};

#[derive(Debug)]
pub struct Bible
{
    pub books: Vec<Book>
}

impl Bible 
{
    pub fn from_md(text: &str) -> Result<Self, String>
    {
        let ast = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();
        match parse_book(&ast.children().unwrap(), 0) 
        {
            Ok(book) => Ok(Self { books: vec![book] }),
            Err(err) => Err(err),
        }
    }
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

fn parse_book(ast: &Vec<Node>, id: u32) -> Result<Book, String>
{
    let Some(Node::Heading(h)) = ast.first() else { return Err("Expected a book heading".into()); };
    let Some(Node::Text(book_name)) = h.children.first() else { return Err("Expected a book name".into()); };

    let mut i = 1;
    let mut chapters = vec![];

    loop 
    {
        match parse_chapter(ast, &mut i)
        {
            Ok(Some(chapter)) => 
            { 
                chapters.push(chapter)
            },
            Ok(None) => break,
            Err(err) => return Err(err),
        }
    };

    Ok(Book {
        name: book_name.value.clone(),
        chapters,
        id,
    })
}

fn parse_chapter(ast: &Vec<Node>, i: &mut usize) -> Result<Option<Chapter>, String>
{
    if *i >= ast.len() { return Ok(None) }

    let Node::Heading(h) = &ast[*i] else { return Err("Expected a chapter heading".into()); };
    *i += 1;

    let Some(Node::Text(t)) = h.children.first() else { return Err("Expected a chapter title".into()) };
    let Some((_, n)) = t.value.rsplit_once(char::is_whitespace) else { return Err("Expected a verse number and text".into()) };
    let Ok(n) = n.parse() else { return Err("Expected a chapter number".into()) };

    let verses = parse_verses(ast, i);

    match verses
    {
        Ok(vs) => Ok(Some(Chapter {
            number: n,
            verses: vs
        })),
        Err(err) => Err(err)
    }
}

fn parse_verses(ast: &Vec<Node>, i: &mut usize) -> Result<Vec<Verse>, String>
{
    let mut verses = vec![];
    while let Some(Node::Paragraph(p)) = ast.get(*i)
    {
        let Some(Node::Text(t)) = p.children.first() else { return Err("Expected verse text.".into()) };
        let Some((n, v)) = t.value.split_once(char::is_whitespace) else { return Err("Expected a verse number and text".into()) };
        let Ok(n) = n.parse() else { return Err("Expected a verse number".into()) };

        verses.push(Verse {
            number: n,
            text: String::from(v)
        });

        *i += 1;
    }

    Ok(verses)
}