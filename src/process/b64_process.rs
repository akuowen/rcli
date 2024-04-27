use std::{fs::File, io::Read};

use crate::{B64Format, B64Ops};
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
pub async fn process_base64(b64_opts: &B64Ops) -> Result<()> {
    match b64_opts {
        B64Ops::Decode(b64_opts) => process_decode(&b64_opts.input, b64_opts.format).await,
        B64Ops::Encode(b64_opts) => process_encode(&b64_opts.input, b64_opts.format).await,
    }
}
/// 解码
pub async fn process_decode(input: &str, format: B64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    println!("{:?}", buf);

    let decode = match format {
        B64Format::Standard => STANDARD.decode(buf.trim())?,
        B64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf.trim())?,
    };

    let result = String::from_utf8(decode)?;
    print!("{:?}", result);
    Ok(())
}

/// 编码
pub async fn process_encode(input: &str, format: B64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let encoded = match format {
        B64Format::Standard => STANDARD.encode(&buf),
        B64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_encode() {
        let input = "Cargo.toml";
        let format = B64Format::Standard;
        assert!(process_encode(input, format).await.is_ok());
    }

    #[tokio::test]
    async fn test_process_decode() {
        let input = "assets/b64.txt";
        let format = B64Format::Standard;
        process_decode(input, format).await.unwrap()
    }
}
