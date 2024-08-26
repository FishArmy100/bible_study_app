
pub struct Word
{
    pub word: String,
    pub italicized: bool,
}

pub struct Verse
{
    pub words: Vec<Word>
}

pub struct Chapter
{
    pub verses: Vec<Verse>
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
                        italicized
                    });
                }
            }

            word.push(c);
        }

        if word.len() > 0 
        { 
            words.push(Word {
                word: std::mem::replace(&mut word, String::new()),
                italicized
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

