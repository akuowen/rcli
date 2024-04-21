use rcli::{process_csv, process_passgen, RCli, SubCommand};

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli: RCli = RCli::parse();

    match cli.command {
        SubCommand::Csv(cli) => {
            // 如果是output的话 直接clone
            let output: String = if let Some(output) = cli.output {
                output.clone()
            } else {
                // 否则 对format进行转换
                format!("result.{}", cli.format)
            };
            process_csv(&cli.input, output.as_str(), cli.format)?
        }
        SubCommand::GenPass(cli) => {
            process_passgen(&cli)?;

            // println!("generator pass {:?}", cli)
        }
    }
    Ok(())
}
