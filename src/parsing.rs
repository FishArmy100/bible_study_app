use regex::Regex;

use crate::bible::*;

// format: Book Name 10:5 rest of the verse text

pub fn parse_verse(text: &str) -> Verse
{
    let mut italicized = false;
    let mut word = String::new();
    let mut words = vec![];
    for i in 0..text.len()
    {
        let c = text.chars().nth(i).unwrap();
        if c == '[' { italicized = true; continue; }
        if c == ']' { italicized = false; continue; } 

        if c.is_whitespace()
        {
            if word.len() > 0 
            { 
                words.push(Word {
                    word: std::mem::replace(&mut word, String::new()),
                    italicized,
                    red: false,
                });
            }
        }

        word.push(c);
    }

    if word.len() > 0 
    { 
        words.push(Word {
            word: std::mem::replace(&mut word, String::new()),
            italicized,
            red: false,
        });
    }

    Verse { words }
}

pub fn parse_bible(text: &str) -> Bible 
{
    let pattern = Regex::new(r"\s*([1-2]?\s*.*?)\s*([0-9]*):([0-9]*)\s*(.*)\n?").unwrap();
    for (_, [book, chapter, verse, text]) in pattern.captures_iter(text).map(|c| c.extract())
    {
        println!("{}", text);
        break;
    };

    

    let mut books: Vec<(String, Vec<Chapter>)> = vec![];

    Bible { name: "".into(), books: vec![] }
}

