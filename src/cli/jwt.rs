use clap::Parser;

#[derive(Debug, Parser)]
pub enum JwtOpts {
    Sign(JwtSignOpt),
    Verify(JwtVerifyOpt),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpt {
    #[arg(long, help = "subject")]
    pub sub: String,
    #[arg(long, default_value = "-", help = "audience")]
    pub aud: String,
    #[arg(long, default_value_t = 14, help = "expiration time in days")]
    pub exp: i32,
    #[arg(long, help = "private key")]
    pub secret: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpt {
    #[arg(short, long, help = "token")]
    pub token: String,
}
