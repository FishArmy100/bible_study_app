use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Bible
{
    pub name: String,
    pub description: Option<String>,
    pub copyright: Option<String>,
    pub books: Vec<Book>
}

impl Bible
{
    pub fn to_json(&self, mode: JsonMode) -> Result<String, serde_json::Error> 
    {
        match mode
        {
            JsonMode::Dirty => serde_json::to_string(self),
            JsonMode::Pretty => serde_json::to_string_pretty(self),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error>
    {
        serde_json::from_str(json)
    }

    pub fn to_compressed_json(&self, level: CompressionLevel) -> Result<Vec<u8>, String>
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

    pub fn from_compressed_json(data: &[u8]) -> Result<Self, String>
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Book
{
    pub name: String,
    pub testament: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter
{
    pub verses: Vec<Verse>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Verse 
{
    pub text: String,
}
