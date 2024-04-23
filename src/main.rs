use clap::Parser;
use rcli::{process_base64, process_csv, process_passgen, process_text, RCli, SubCommand};

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
        SubCommand::Base64(b64_opts) => {
            process_base64(&b64_opts)?;
        }
        SubCommand::Text(text_opts) => process_text(&text_opts)?,
    }
    Ok(())
}
