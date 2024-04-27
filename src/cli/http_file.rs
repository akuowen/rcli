use clap::Parser;

#[derive(Debug, Parser)]
pub struct HttpFileOpts {
    #[arg(short, long, default_value = "8080", help = "port number")]
    pub port: u16,
    #[arg(short, long, default_value = ".", help = "file path")]
    pub file_path: String,
}
