use std::{fmt, str::FromStr};

use clap::Parser;

#[derive(Debug, Parser)]
pub enum B64Ops {
    #[command(name = "decode", about = "decode base64 string")]
    Decode(B64DecodeOps),
    #[command(name = "encode", about = "encode base64 string")]
    Encode(B64EncodeOps),
}

#[derive(Debug, Clone, Copy)]
pub enum B64Format {
    Standard,
    UrlSafe,
}

#[derive(Debug, Parser)]
pub struct B64DecodeOps {
    #[arg(short, long)]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: B64Format,
}

#[derive(Debug, Parser)]
pub struct B64EncodeOps {
    #[arg(short, long)]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: B64Format,
}

fn parse_base64_format(format: &str) -> Result<B64Format, anyhow::Error> {
    match format {
        "standard" => Ok(B64Format::Standard),
        "urlsafe" => Ok(B64Format::UrlSafe),
        _ => anyhow::bail!("not support format"),
    }
}

impl FromStr for B64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(B64Format::Standard),
            "urlsafe" => Ok(B64Format::UrlSafe),
            _ => anyhow::bail!("not support format"),
        }
    }
}

impl From<B64Format> for &'static str {
    fn from(format: B64Format) -> Self {
        match format {
            B64Format::Standard => "standard",
            B64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for B64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
