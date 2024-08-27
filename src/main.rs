mod html;
mod bible;
mod parsing;

use itertools::Itertools;
use parsing::parse_bible;
use web_view::*;

fn main() {
    // let html_content = include_str!("../assets/page.html");

    
    let data = parse_bible(include_str!("../assets/kjv.txt")).unwrap();
    let html_content = html::build_chapter_html(&data.books[19].chapters[2], "Proverbs", 3).unwrap();

    web_view::builder()
        .title("Bible App")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}