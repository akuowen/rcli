use clap::Parser;

#[derive(Debug, Parser)]
pub enum JwtOpts {
    Sign(JwtSignOpt),
    Verify(JwtVerifyOpt),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpt {}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpt {}
