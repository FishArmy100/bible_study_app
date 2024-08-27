mod html;
mod bible;
mod parsing;
pub mod bible_view;

use bible::ChapterRef;
use parsing::parse_bible;
use serde::{Deserialize, Serialize};
use web_view::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd 
{
    SelectChapter {
        chapter: ChapterRef,
    },
    Test,
}

fn main() {
    let bible = parse_bible(include_str!("../assets/kjv.txt")).unwrap();
    let html_content = bible_view::build_view_page(&bible).unwrap();

    web_view::builder()
        .title("Bible App")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            println!("{}", arg);
            match serde_json::from_str::<Cmd>(arg).unwrap() 
            {
                Cmd::SelectChapter { chapter } => println!("Selected {} chapter {}", chapter.book, chapter.chapter),
                Cmd::Test => println!("Ran testing command"),
            }

            Ok(())
        })
        .run()
        .unwrap();
}