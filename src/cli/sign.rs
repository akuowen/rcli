use clap::{arg, Parser};

#[derive(Debug, Parser)]
pub enum TextSignVerifyOpt {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Encrypt(SignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Decrypt(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}
