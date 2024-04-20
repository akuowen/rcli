use std::path::Path;
use clap::{Parser, Subcommand};
/*
   see {@link https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html}
    rCli csv -i filePath -o  xxx --header -d ''
 */
#[derive(Parser,Debug)]
#[command(name = "rCli",version, about,author, long_about = None)]
pub struct RCli {

    #[command(SubCommand)]
    pub command: Option<Commands>,
}

pub enum SubCommand {
    #[command(name = "csv",about = "convert or show csv file ")]
    Csv(Commands)

}
#[derive(Subcommand,Debug)]
pub struct  Commands {
    /// does testing things
    ///
    #[arg(arg,short="i",long,value_parser=verity_input_path)]
    pub input: String,
    pub output: String,
    pub header: bool,
    pub delimiter: char

}


fn verity_input_path(file_path: &str) ->Result<String,&'static str>{
    if Path::new(file_path).exists() {
        Ok(file_path.into())
    }else {
        Err("file does not exists")
    }
}