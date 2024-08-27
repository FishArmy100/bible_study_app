use crate::bible::*;
use html_builder::*;
use std::fmt::{Write, Error};

pub type HtmlResult = Result<String, Error>;

pub fn build_chapter_html(chapter: &Chapter, book_name: &str, chapter_number: u32) -> HtmlResult
{
    let mut buffer = Buffer::new();
    buffer.doctype();
    let mut html = buffer.html().attr("lang='en'");

    let mut head = html.head();
    write!(head.title(), "Bible App")?;
    head.style().write_str(include_str!("../assets/page.css"))?;

    let mut body = html.body();
    {
        body.h1().attr("chapter-header").write_str(&format!("{book_name} chapter {chapter_number}:"))?;
    }
    {
        let mut div = body.div().attr("class=scrollable-list");
        let mut list = div.ol();

        for v in &chapter.verses
        {
            build_verse_text(&mut list.li(), &v)?
        }
    }

    Ok(buffer.finish())
}

fn build_verse_text(node: &mut Node, verse: &Verse) -> Result<(), Error>
{
    for w in &verse.words
    {
        if w.italicized
        {
            node.i().write_str(&w.word)?;
        }
        else
        {
            node.write_str(&w.word)?;
        }
    }

    Ok(())
}