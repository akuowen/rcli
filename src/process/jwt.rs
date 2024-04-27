use crate::JwtOpts;
use anyhow::{Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i32,
}

pub fn process_jwt(jwt_opts: &JwtOpts) -> Result<()> {
    match jwt_opts {
        JwtOpts::Sign(sign) => {
            let secret = sign.secret.clone();
            let my_claims = Claims {
                sub: sign.sub.clone(),
                company: sign.aud.clone(),
                exp: sign.exp,
            };
            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )?;
            println!("sign {:?}", token);
        }
        JwtOpts::Verify(verify) => {
            print!("token {:?}", &verify.token);
            let mut validation: Validation = Validation::new(Algorithm::HS256);
            validation.leeway = 0;
            validation.validate_exp = true;
            let token_message = decode::<Claims>(
                &verify.token,
                &DecodingKey::from_secret("akuowen".as_ref()),
                &validation,
            );
            print!("verify {:?}", token_message)
        }
    }
    Ok(())
}

pub async fn process_sign(secret: &str, sub: &str, aud: &str, exp: i32) -> Result<()> {
    let my_claims = Claims {
        sub: sub.to_string(),
        company: aud.to_string(),
        exp,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    println!("sign {:?}", token);
    Ok(())
}

pub async fn process_verify(secret: &str, token: &str) -> Result<()> {
    let mut validation: Validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0;
    validation.validate_exp = true;
    let token_message = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );
    print!("verify {:?}", token_message);
    Ok(())
}
