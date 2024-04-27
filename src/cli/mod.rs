mod b64;
mod csv_opt;
mod gen_passwd;
mod http_file;
mod jwt;
mod sign;
mod text;

pub use b64::{B64DecodeOps, B64EncodeOps, B64Format, B64Ops};
pub use clap::Parser;
pub use csv_opt::Commands;
pub use csv_opt::Format;
use enum_dispatch::enum_dispatch;
use fmt::Debug;
pub use gen_passwd::GenPassOps;
pub use http_file::HttpFileOpts;
pub use jwt::{JwtOpts, JwtSignOpt, JwtVerifyOpt};
pub use sign::{SignOpts, TextSignVerifyOpt, VerifyOpts};
use std::fmt;
use std::path::Path;
pub use text::{TextFormat, TextOps, TextSignOpts, TextVerifyOpts};
#[derive(Debug, Parser)]
#[command(name = "rCli",version, about,author, long_about = None)]
pub struct RCli {
    #[command(subcommand)]
    // #[command(subcommandOpt)]
    pub opt: self::SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    /// csv转json/yaml/toml
    #[command(name = "csv", about = "convert or show csv file ")]
    Csv(Commands),

    /// 生成密码
    #[command(name = "genpasswd", about = "generator password")]
    GenPass(GenPassOps),

    /// base64 encode decode
    #[command(subcommand, about = "base64 encode decode")]
    Base64(B64Ops),

    ///
    #[command(subcommand, about = "text sign verify")]
    Text(TextOps),
    /// rCli jwt sign --sub acme --aud device1 --exp 14d
    /// rCli jwt verify -t
    /// jwt 生成和验证
    #[command(subcommand, about = "jwt sign verify")]
    Jwt(JwtOpts),
    /// http file serve
    #[command(name = "http", about = "download file from http")]
    Http(HttpFileOpts),
    /// chacha20poly1305 加解密
    #[command(subcommand, about = "text sign verify")]
    Sign(TextSignVerifyOpt),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
