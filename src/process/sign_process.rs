use crate::{SignOpts, VerifyOpts};
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};

/// 验证

/// 加密

pub async fn sign(sign_opts: &SignOpts) -> Result<std::string::String> {
    let input = &sign_opts.input;
    let sign_key = Key::clone_from_slice(&str_to_u8_32(&sign_opts.key));
    let cipher = ChaCha20Poly1305::new(&sign_key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    println!("sign nonce:{:?}", nonce.to_vec());
    let ciphertext = cipher
        .encrypt(&nonce, input.as_ref())
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    println!("sign ciphertext:{:?}", ciphertext.to_vec());
    let mut result = Vec::new();
    result.extend_from_slice(&nonce[0..6]);
    result.extend(&ciphertext);
    result.extend(&nonce[6..]);
    println!("sign result:{:?}", result.to_vec());
    // println!("{:?}", URL_SAFE_NO_PAD.encode(result));
    Ok(URL_SAFE_NO_PAD.encode(result))
}

pub async fn verify(verify_opts: &VerifyOpts) -> Result<()> {
    println!("verify token{:?}", &verify_opts.input);
    // let token_vec = URL_SAFE_NO_PAD.decode(&verify_opts.input);
    let ciphertext_bytes = URL_SAFE_NO_PAD.decode(&verify_opts.input)?;
    println!("verify result:{:?}", ciphertext_bytes);
    let mut nonce = [0u8; 12];
    nonce[0..6].copy_from_slice(&ciphertext_bytes[0..6]);
    nonce[6..12].copy_from_slice(&ciphertext_bytes[ciphertext_bytes.len() - 6..]);
    println!(" verify nonce:{:?}", nonce);
    let key_str = str_to_u8_32(&verify_opts.key);
    let key = Key::clone_from_slice(&key_str);
    let cipher = ChaCha20Poly1305::new(&key);
    let text = &ciphertext_bytes[6..ciphertext_bytes.len() - 6];
    println!("verify text:{:?}", text);
    let plaintext = cipher
        .decrypt(<&Nonce>::from(&nonce), text)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    println!("{:?}", std::string::String::from_utf8(plaintext).unwrap());
    Ok(())
}

fn str_to_u8_32(s: &str) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = s.as_bytes();

    for (&x, p) in bytes.iter().zip(array.iter_mut()) {
        *p = x;
    }

    array
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_all() {
        let text_sign_opts = SignOpts {
            key: "akuo".to_string(),
            input: "akuowen".to_string(),
        };
        let sign_result = sign(&text_sign_opts).await.unwrap();

        let verify_opts = VerifyOpts {
            key: "akuo".to_string(),
            input: sign_result,
        };

        let _ = verify(&verify_opts).await;
    }

    #[tokio::test]
    async fn test_sign() {
        let text_sign_opts = SignOpts {
            key: "akuo".to_string(),
            input: "akuowen".to_string(),
        };
        let _ = sign(&text_sign_opts).await;
    }

    #[tokio::test]
    async fn test_verify() {
        let verify_opts = VerifyOpts {
            key: "aku".to_string(),
            input: "N47xi0WZxnwl5DtZZKmHQ4Ioqy-K20RhNqVQ1tjCp8oJzbs".to_string(),
        };

        let _ = verify(&verify_opts).await;
    }
}
