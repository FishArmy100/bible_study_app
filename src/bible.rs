use markdown::mdast::{List, Node};

pub struct Bible
{
    pub books: Vec<Book>
}

impl Bible 
{
    pub fn from_md(text: &str) -> Result<Self, Vec<String>>
    {
        let ast = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();
        match parse_book(&ast.children().unwrap(), 0) 
        {
            Ok(book) => Ok(Self { books: vec![book] }),
            Err(err) => Err(err),
        }
    }
}

pub struct Book
{
    pub id: u32,
    pub name: String,
    pub chapters: Vec<Chapter>
}

pub struct Chapter
{
    pub number: u32,
    pub verses: Vec<Verse>
}

pub struct Verse 
{
    pub number: u32,
    pub text: String,
}

fn parse_book(ast: &Vec<Node>, id: u32) -> Result<Book, Vec<String>>
{
    let Some(Node::Heading(h)) = ast.first() else { return Err(vec!["Expected a book heading".into()]); };
    let Some(Node::Text(book_name)) = h.children.first() else { return Err(vec!["Expected a book name".into()]); };

    let mut i = 1;
    let chapters = loop 
    {
        let mut chapters = vec![];
        match parse_chapter(ast, &mut i, chapters.len() as u32 + 1)
        {
            Ok(Some(chapter)) => chapters.push(chapter),
            Ok(None) => break chapters,
            Err(err) => return Err(err),
        }
    };

    Ok(Book {
        name: book_name.value.clone(),
        chapters,
        id,
    })
}

fn parse_chapter(children: &Vec<Node>, i: &mut usize, chapter_count: u32) -> Result<Option<Chapter>, Vec<String>>
{
    let Node::Heading(h) = &children[*i] else { return Err(vec!["Expected a chapter heading".into()]); };

    if let Some(Node::Text(t)) = h.children.first() 
    {
        if t.value == "eof" 
        { 
            *i += 1;
            return Ok(None) // end of book
        }
    }

    *i += 1;
    let Node::List(l) = &children[*i] else { return Err(vec!["Expected a verse list".into()]); };
    let verses = parse_verses(l);
    *i += 1;

    match verses
    {
        Ok(vs) => Ok(Some(Chapter {
            number: chapter_count,
            verses: vs
        })),
        Err(err) => Err(err)
    }
}

fn parse_verses(l: &List) -> Result<Vec<Verse>, Vec<String>>
{
    let mut verse_number = 1;
    l.children.iter().map(|c| {
        let Node::ListItem(i) = c else { return Err(String::from("Expected a list item")) };
        match i.children.first()
        {
            Some(Node::Text(t)) => {
                let verse = Verse {
                    number: verse_number,
                    text: t.value.clone()
                };
                verse_number += 1;
                Ok(verse)
            },
            _ => Err(String::from("Expected a verse"))
        }
    }).fold(Result::<Vec<Verse>, Vec<String>>::Ok(vec![]), |i, v| {
        match i 
        {
            Ok(mut vs) => if v.is_ok() {
                vs.push(v.ok().unwrap());
                Ok(vs)
            } 
            else 
            {
                Err(vec![v.err().unwrap()])
            },
            Err(mut errs) => if v.is_err() {
                errs.push(v.err().unwrap());
                Err(errs)
            }
            else 
            {
                Err(errs)
            }
        }
    })
}