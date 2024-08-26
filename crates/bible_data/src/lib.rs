use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Word
{
    pub word: String,
    pub italicized: bool,
    pub red: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verse
{
    pub words: Vec<Word>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter
{
    pub verses: Vec<Verse>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book 
{
    pub name: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bible
{
    pub name: String,
    pub books: Vec<Book>,
}

pub fn chapter_from_text(text: &str) -> Chapter
{
    let verse_texts = text.split('\n');
    let mut verses = vec![];
    for verse in verse_texts
    {

        let mut italicized = false;
        let mut word = String::new();
        let mut words = vec![];
        for i in 0..verse.len()
        {
            let c = verse.chars().nth(i).unwrap();
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

        verses.push(Verse {
            words
        });
    }

    Chapter { 
        verses 
    }
}

