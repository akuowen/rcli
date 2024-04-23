use std::{fmt, path::Path, str::FromStr};

use clap::{arg, Parser};

#[derive(Debug, Parser)]
pub enum TextOps {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser=verify_file)]
    pub input: String,
    #[arg(short,long,value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long,value_parser=verify_file, default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser=verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextFormat::Blake3),
            "ed25519" => Ok(TextFormat::Ed25519),
            _ => anyhow::bail!("not support format"),
        }
    }
}

impl From<TextFormat> for &'static str {
    fn from(value: TextFormat) -> Self {
        match value {
            TextFormat::Blake3 => "blake3",
            TextFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

// fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
//     // if input is "-" or file exists
//     let p = Path::new(path);
//     if p.exists() && p.is_dir() {
//         Ok(path.into())
//     } else {
//         Err("Path does not exist or is not a directory")
//     }
// }

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
