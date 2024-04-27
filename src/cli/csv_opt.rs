use std::{fmt, path::Path, str::FromStr};

use crate::{process_csv, CmdExecutor};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Commands {
    /// does testing things
    ///
    #[arg(short,long,value_parser=verity_input_path)]
    pub input: String,
    #[arg(short, long)]
    pub output: String,
    #[arg(short = 'H', long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long,default_value="json" ,value_parser = format_parse)]
    pub format: Format,
}

impl CmdExecutor for Commands {
    async fn execute(self) -> anyhow::Result<()> {
        process_csv(&self.input, self.output.as_str(), self.format).await
    }
}

fn verity_input_path(file_path: &str) -> Result<String, &'static str> {
    if Path::new(file_path).exists() {
        Ok(file_path.into())
    } else {
        Err("file does not exists")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

fn format_parse(file_format: &str) -> Result<Format, anyhow::Error> {
    file_format.parse().map_err(|e: anyhow::Error| e)
}

impl From<Format> for &'static str {
    fn from(value: Format) -> Self {
        match value {
            Format::Json => "json",
            Format::Yaml => "yaml",
            Format::Toml => "toml",
        }
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            _ => anyhow::bail!("not support format"),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
