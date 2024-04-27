use crate::{process_passgen, CmdExecutor};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOps {
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOps {
    async fn execute(self) -> anyhow::Result<()> {
        process_passgen(&self).await
    }
}
