use std::path::Path;
use clap::{Parser};
/*
   see {@link https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html}
    rCli csv -i filePath -o  xxx --header -d ''
 */
#[derive(Parser,Debug)]
#[command(name = "rCli",version, about,author, long_about = None)]
pub struct RCli {

    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv",about = "convert or show csv file ")]
    Csv(Commands)

}

#[derive(Debug, Parser)]
pub struct  Commands {
    /// does testing things
    ///
    #[arg(short,long,value_parser=verity_input_path)]
    pub input: String,
    #[arg(short, long, default_value = "result.json")]
    pub output: String,
    #[arg(short='H',long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char

}


fn verity_input_path(file_path: &str) ->Result<String,&'static str>{
    if Path::new(file_path).exists() {
        Ok(file_path.into())
    }else {
        Err("file does not exists")
    }
}