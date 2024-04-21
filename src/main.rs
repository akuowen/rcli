use rcli::{process_csv, RCli, SubCommand};

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = RCli::parse();
    match cli.command {
        SubCommand::Csv(cli) => process_csv(&cli.input, &cli.output)?,
    }
    Ok(())
}
