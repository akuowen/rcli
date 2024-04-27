use clap::Parser;
use rcli::{
    process_base64, process_csv, process_http_file, process_jwt, process_passgen, process_text,
    sign, verify, RCli, SubCommand, TextSignVerifyOpt,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: RCli = RCli::parse();

    tracing_subscriber::fmt::init();
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
        SubCommand::Jwt(jwt_opts) => process_jwt(&jwt_opts)?,
        SubCommand::Http(http_opts) => process_http_file(&http_opts).await?,
        SubCommand::Sign(sign_opts) => match sign_opts {
            TextSignVerifyOpt::Encrypt(encrypt) => println!("加密为{:?}", sign(&encrypt)?),
            TextSignVerifyOpt::Decrypt(decrypt) => verify(&decrypt)?,
        },
    }
    Ok(())
}
