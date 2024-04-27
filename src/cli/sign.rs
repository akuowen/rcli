use crate::{sign, verify, CmdExecutor};
use clap::{arg, Parser};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("sign result:{:?}", sign(&self).await);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

impl CmdExecutor for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        verify(&self).await
    }
}
