mod b64;
mod csv_opt;
mod gen_passwd;
mod text;

use std::path::Path;

pub use b64::{B64Format, B64Ops};
use clap::Parser;
pub use csv_opt::Commands;
pub use csv_opt::Format;
pub use gen_passwd::GenPassOps;
pub use text::{TextFormat, TextOps, TextSignOpts, TextVerifyOpts};

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

    #[command(name = "genpasswd", about = "generator password")]
    GenPass(GenPassOps),

    #[command(subcommand)]
    Base64(B64Ops),

    #[command(subcommand)]
    Text(TextOps),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
