use std::io::Read;

use anyhow::{Ok, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{get_content, get_reader, TextFormat, TextOps};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
/// 签名

pub trait TextSigner<T>
where
    T: Read,
{
    fn sign(&self, reader: T) -> Result<Vec<u8>>;
}

/// 验签
pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}
pub struct Blake3 {
    pub key: [u8; 32],
}

impl<T: Read> TextSigner<T> for Blake3 {
    fn sign(&self, mut reader: T) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        // 实现细节
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes() == sig)
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

pub struct Ed25519Sign {
    pub key: SigningKey,
}

impl<T: Read> TextSigner<T> for Ed25519Sign {
    fn sign(&self, mut reader: T) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl Ed25519Sign {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }
}

pub struct Ed25519Ver {
    pub key: VerifyingKey,
}

impl TextVerifier for Ed25519Ver {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl Ed25519Ver {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub fn process_text(text_opts: &TextOps) -> Result<()> {
    match text_opts {
        TextOps::Sign(text_sign_opts) => {
            print!("{:?}", text_sign_opts);
            let key_content = get_content(&text_sign_opts.key)?;
            let signer: Box<dyn TextSigner<_>> = match text_sign_opts.format {
                TextFormat::Blake3 => Box::new(Blake3::try_new(key_content)?),
                TextFormat::Ed25519 => Box::new(Ed25519Sign::try_new(key_content)?),
            };

            println!(
                "{:?}",
                URL_SAFE_NO_PAD.encode(signer.sign(get_reader(&text_sign_opts.input)?)?)
            );
            Ok(())
        }
        TextOps::Verify(text_verify_opts) => {
            let key_content = get_content(&text_verify_opts.input)?;
            let decoded = URL_SAFE_NO_PAD.decode(&text_verify_opts.sig)?;

            let verify: Box<dyn TextVerifier> = match text_verify_opts.format {
                TextFormat::Blake3 => Box::new(Blake3::try_new(key_content)?),
                TextFormat::Ed25519 => Box::new(Ed25519Ver::try_new(key_content)?),
            };
            let mut reader = get_reader(&text_verify_opts.input)?;

            let result = verify.verify(&mut reader, &decoded);
            println!("{:?}", result);
            Ok(())
        }
    }
}
