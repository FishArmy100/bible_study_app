use markdown::{mdast::Node, unist::Position};

use bible_file_format::{Bible, Book, Chapter, Verse};

pub enum MdTestament
{
    Old,
    New,
}

impl ToString for MdTestament
{
    fn to_string(&self) -> String 
    {
        match self
        {
            MdTestament::Old => String::from("Old"),
            MdTestament::New => String::from("New"),
        }
    }
}

pub struct MdBookFile
{
    pub path: String,
    pub src: String,
    pub testament: MdTestament,
}

pub fn bible_from_md(files: &[MdBookFile], name: String) -> Result<Bible, String>
{
    let mut books = vec![];
    for file in files
    {
        let ast = markdown::to_mdast(&file.src, &markdown::ParseOptions::default()).unwrap();
        match parse_book(&ast.children().unwrap(), file.testament.to_string(), &file.path) 
        {
            Ok(book) => books.push(book),
            Err(err) => return Err(err),
        }
    };

    Ok(Bible {
        name,
        description: None,
        copyright: None,
        books,
    })
}

fn parse_book(ast: &Vec<Node>, testament: String, debug_file_path: &str) -> Result<Book, String>
{
    let Some(Node::Heading(h)) = ast.first() else { return Err(format_error_message("Expected a book heading", &debug_file_path, None)); };
    let Some(Node::Text(book_name)) = h.children.first() else { return Err(format_error_message("Expected a book name", &debug_file_path, h.position.clone())); };

    let mut i = 1;
    let mut chapters = vec![];

    loop 
    {
        match parse_chapter(ast, &mut i, debug_file_path)
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
        testament,
        chapters,
    })
}

fn parse_chapter(ast: &Vec<Node>, i: &mut usize, debug_file_path: &str) -> Result<Option<Chapter>, String>
{
    if *i >= ast.len() { return Ok(None) }

    let Node::Heading(h) = &ast[*i] else { return Err(format_error_message("Expected a chapter heading", &debug_file_path, ast[*i].position().cloned())); };
    *i += 1;

    let Some(Node::Text(t)) = h.children.first() else { return Err(format!("Expected a chapter title")) };
    
    if t.value.trim() == "eof" { return Ok(None) } // reached the end of the file

    let Some((_, n)) = t.value.rsplit_once(char::is_whitespace) else { return Err(format_error_message("Expected a verse number and text", &debug_file_path, t.position.clone())) };
    let Ok(_): Result<u32, _> = n.parse() else { return Err(format_error_message("Expected a chapter number", &debug_file_path, t.position.clone())) };

    let verses = parse_verses(ast, i, debug_file_path);

    match verses
    {
        Ok(vs) => Ok(Some(Chapter {
            verses: vs
        })),
        Err(err) => Err(err)
    }
}

fn parse_verses(ast: &Vec<Node>, i: &mut usize, debug_file_path: &str) -> Result<Vec<Verse>, String>
{
    let mut verses = vec![];
    while let Some(Node::Paragraph(p)) = ast.get(*i)
    {
        let Some(Node::Text(t)) = p.children.first() else { return Err(format_error_message("Expected verse text.", debug_file_path, p.position.clone())) };
        let Some((n, v)) = t.value.split_once(char::is_whitespace) else { return Err(format_error_message("Expected a verse number and text", debug_file_path, t.position.clone())) };
        let Ok(_): Result<u32, _> = n.parse() else { return Err(format_error_message("Expected a verse number", debug_file_path, t.position.clone())) };

        verses.push(Verse {
            text: String::from(v)
        });

        *i += 1;
    }

    Ok(verses)
}

fn format_error_message(message: &str, file: &str, position: Option<Position>) -> String 
{
    match position
    {
        Some(pos) => format!("{}\nfile = {}\npos = {:?}", message, file, pos),
        None => format!("{}\nfile = {}", message, file),
    }
}