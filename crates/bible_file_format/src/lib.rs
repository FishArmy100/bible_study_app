use bible::ChapterSave;
use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

pub mod notes;
pub mod bible;

/// **NOTE: all values are i**
#[repr(C)]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct TextRange
{
    pub book_index: u8,
    pub chapter_index: u8,
    pub start_verse_index: u16,
    pub start_word_index: u16,
    pub end_verse_index: u16,
    pub end_word_index: u16,
}

impl TextRange
{
    pub fn word(book_index: u8, chapter_index: u8, verse: u16, word: u16) -> Self 
    {
        Self 
        {
            book_index,
            chapter_index,
            start_verse_index: verse,
            start_word_index: word,
            end_verse_index: verse,
            end_word_index: word,
        }
    }
}

pub enum JsonMode
{
    Pretty,
    Dirty,
}

pub enum CompressionLevel
{
    Low,
    Medium,
    High
}

pub trait JsonSerde : serde::Serialize + for<'a> serde::Deserialize<'a>
{
    fn to_json(&self, mode: JsonMode) -> Result<String, String>
    {
        match mode
        {
            JsonMode::Dirty => serde_json::to_string(self),
            JsonMode::Pretty => serde_json::to_string_pretty(self),
        }.map_err(|e| e.to_string())
    }

    fn from_json(json: &str) -> Result<Self, String>
    {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    fn to_compressed_json(&self, level: CompressionLevel) -> Result<Vec<u8>, String>
    {
        let json = match self.to_json(JsonMode::Dirty)
        {
            Ok(json) => json,
            Err(e) => return Err(e.to_string())
        };

        let compression_level = match level
        {
            CompressionLevel::Low => 1,
            CompressionLevel::Medium => 5,
            CompressionLevel::High => 10,
        };

        Ok(compress_to_vec(json.as_bytes(), compression_level))
    }

    fn from_compressed_json(data: &[u8]) -> Result<Self, String>
    {
        let json = match decompress_to_vec(data)
        {
            Ok(ok) => match String::from_utf8(ok) 
            {
                Ok(ok) => ok,
                Err(err) => return Err(err.to_string()),
            },
            Err(err) => return Err(err.to_string()),
        };

        match Self::from_json(&json)
        {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.to_string()),
        }
    }
}

