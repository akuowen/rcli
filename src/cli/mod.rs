mod csv_opt;
mod gen_passwd;

use clap::Parser;
pub use csv_opt::Commands;
pub use csv_opt::Format;
pub use gen_passwd::GenPassOps;

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
}
