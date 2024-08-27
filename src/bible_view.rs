use std::fmt::{Error, Write};

use html_builder::*;
use itertools::Itertools;
use crate::{bible::*, html::*, Cmd};

pub fn build_view_page(bible: &Bible) -> HtmlResult
{
    crate::html::build_page(PageDesc { 
        styles: &[include_str!("../assets/page.css")], 
        scripts: &[include_str!("../assets/util.js")], 
        body_fn: |body| {
            chapter_selection_dropdown(body, bible)?;
            Ok(())
        } 
    })
}


fn chapter_selection_dropdown(node: &mut Node, bible: &Bible) -> Result<(), Error>
{
    let mut dropdown = node.div().attr("class=\"dropdown\"");
    dropdown.button().attr("class=\"dropbtn\"").write_str("Dropdown")?;

    let mut content_div = dropdown.div().attr("class=\"dropdown-content\"");
    let mut content = content_div.ul();

    let result: Result<Vec<_>, _> = bible.books.iter().map(|b| {
        b.chapters.iter().enumerate().map(|(idx, _)| ChapterRef {
            book: b.name.clone(),
            chapter: idx as u32 + 1
        })
    }).flatten().map(|chapter| {
        selection_option(&mut content, &chapter)
    }).collect();

    match result
    {
        Ok(_) => {Ok(())},
        Err(e) => Err(e),
    }
}

fn selection_option(node: &mut Node, chapter: &ChapterRef) -> Result<(), Error>
{
    let mut item = node.li();

    let command = format!("onclick=\"invoke({{cmd: 'selectChapter', chapter: {{ book: '{}', chapter: {} }} }})\"", chapter.book, chapter.chapter);
    println!("{command}");
    let option_label = format!("{} {}", chapter.book, chapter.chapter);
    item.button().attr(&command).write_str(&option_label)?;

    Ok(())
}