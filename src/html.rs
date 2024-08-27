use crate::bible::*;
use html_builder::*;
use std::fmt::{Write, Error};

pub type HtmlResult = Result<String, Error>;
pub type HtmlBuildResult = Result<(), Error>;

pub struct PageDesc<'a, F> where F : FnMut(&mut Node) -> HtmlBuildResult
{
    pub styles: &'a [&'a str],
    pub scripts: &'a [&'a str],
    pub body_fn: F,
}

pub fn build_page<F>(mut desc: PageDesc<F>) -> HtmlResult
    where F : FnMut(&mut Node) -> Result<(), Error>
{
    let mut buffer = Buffer::new();
    buffer.doctype();
    let mut html = buffer.html().attr("lang='en'");

    let mut head = html.head();
    let mut style_node = head.style();
    for style in desc.styles
    {
        style_node.write_str(&style)?;
    }

    let mut script_node = head.script();
    for script in desc.scripts
    {
        script_node.write_str(&script)?;
    }

    let mut body = html.body();
    (desc.body_fn)(&mut body)?;

    Ok(buffer.finish())
}

pub fn build_chapter_html(chapter: &Chapter, book_name: &str, chapter_number: u32, body: &mut Node) -> Result<(), Error>
{
    body.h1().attr("chapter-header").write_str(&format!("{book_name} chapter {chapter_number}:"))?;
    let mut div = body.div().attr("class=scrollable-list");
    let mut list = div.ol();

    for v in &chapter.verses
    {
        build_verse_text(&mut list.li(), &v)?
    }

    Ok(())
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