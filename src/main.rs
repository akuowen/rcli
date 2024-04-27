use clap::Parser;

use rcli::CmdExecutor;
use rcli::RCli;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: RCli = RCli::parse();
    tracing_subscriber::fmt::init();
    cli.opt.execute().await?;
    Ok(())
}
