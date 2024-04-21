use clap::Parser;
use core::fmt;
use std::{path::Path, str::FromStr};
/*
  see {@link https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html}
   rCli csv -i filePath -o  xxx --header -d ''
*/
#[derive(Parser, Debug)]
#[command(name = "rCli",version, about,author, long_about = None)]
pub struct RCli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert or show csv file ")]
    Csv(Commands),
}

#[derive(Debug, Parser)]
pub struct Commands {
    /// does testing things
    ///
    #[arg(short,long,value_parser=verity_input_path)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short = 'H', long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long,default_value="json" ,value_parser = format_parse)]
    pub format: Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Json,
    Yaml,
}

fn format_parse(file_format: &str) -> Result<Format, anyhow::Error> {
    file_format.parse().map_err(|e: anyhow::Error| e)
}

fn verity_input_path(file_path: &str) -> Result<String, &'static str> {
    if Path::new(file_path).exists() {
        Ok(file_path.into())
    } else {
        Err("file does not exists")
    }
}

impl From<Format> for &'static str {
    fn from(value: Format) -> Self {
        match value {
            Format::Json => "json",
            Format::Yaml => "yaml",
        }
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            _ => anyhow::bail!("not support format"),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
