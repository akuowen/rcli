use std::{fmt, path::Path, str::FromStr};

use crate::{process_text_sign, process_text_verify, CmdExecutor};
use clap::{arg, Parser};
use enum_dispatch::enum_dispatch;

/// chacha20poly1305 加解密
/// 需要密钥 随机值  和 加密值
/// 由于解密只有一个加密结果和密钥值  所以将 加密步骤的随机数字nonce 以6自己为一部分 拆分放在加密结果数组前后
/// base64(nonce(0-6)  解密结果  nonce(7-12))
/// 以此确保加解密随机值一致
#[enum_dispatch(CmdExecutor)]
#[derive(Debug, Parser)]
pub enum TextOps {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser=verify_file,default_value="-")]
    pub input: String,
    #[arg(short,long,value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_text_sign(&self).await
    }
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

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_text_verify(&self).await
    }
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

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    println!("filename: {}", filename);
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
