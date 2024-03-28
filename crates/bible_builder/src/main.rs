pub mod md_parsing;

use std::path::Path;

use bible_file_format::{CompressionLevel, JsonMode, JsonSerde};
use clap::{Args, Parser, Subcommand};
use md_parsing::{bible_from_md, MdBookFile, MdTestament};

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CompressArg
{
    #[default]
    None,
    Low,
    Medium,
    High
}

#[derive(Debug, Parser)]
struct CLI 
{
    #[arg(short = 'o')]
    output_directory: std::path::PathBuf,

    #[arg(short = 'n')]
    bible_name: String,

    #[arg(short = 'c')]
    #[clap(default_value = "none")]
    compression_level: CompressArg,

    #[command(subcommand)]
    command: ConversionCommands,
}

#[derive(Subcommand, Debug)]
enum ConversionCommands
{
    Md(MarkdownArgs),
    CSV(CSVArgs)
}

#[derive(Args, Debug)]
struct MarkdownArgs
{
    path: std::path::PathBuf,
}

#[derive(Args, Debug)]
struct CSVArgs 
{
    path: std::path::PathBuf,
}

fn main() -> Result<(), String>
{
    let args = CLI::parse();
    match args.command
    {
        ConversionCommands::Md(md_args) => 
        {
            let files = match read_markdown(md_args) {
                Ok(value) => value,
                Err(value) => return value,
            };

            let bible = bible_from_md(&files, args.bible_name.clone())?;

            let data = match args.compression_level
            {
                CompressArg::None => bible.to_json(JsonMode::Pretty).unwrap().into_bytes(),
                CompressArg::Low => bible.to_compressed_json(CompressionLevel::Low).unwrap(),
                CompressArg::Medium => bible.to_compressed_json(CompressionLevel::Medium).unwrap(),
                CompressArg::High => bible.to_compressed_json(CompressionLevel::High).unwrap(),
            };

            match std::fs::write(args.output_directory, data)
            {
                Ok(()) => println!("Successfully Converted the Bible to Json!!!!"),
                Err(err) => return  Err(format!("Failed to write to file because: `{:?}`", err)),
            }
        },
        ConversionCommands::CSV(_) => panic!("CSV files not implemented yet"),
    }

    Ok(())
}

fn read_markdown(md_args: MarkdownArgs) -> Result<Vec<MdBookFile>, Result<(), String>> 
{
    let path = Path::new(&md_args.path);
    if !path.exists() { return Err(Err(String::from("Path path must exist"))); };
    if !path.is_dir() { return Err(Err(String::from("Path must be a directory"))); };
    let files = std::fs::read_dir(path).unwrap().into_iter().filter_map(|p| match p {
        Ok(ok) => Some(ok),
        Err(_) => None
    }).filter(|p| p.path().is_file() && p.path().extension().unwrap() == "md" && !p.path().ends_with("ReadMe.md"))
    .map(|f| {
        MdBookFile {
            path: String::from(f.path().to_str().unwrap()),
            src: std::fs::read_to_string(f.path()).unwrap(),
            testament: MdTestament::Old
        }
    }).collect::<Vec<_>>();
    Ok(files)
}
