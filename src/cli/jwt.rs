use crate::process::process_sign;
use crate::{process_verify, CmdExecutor};
use clap::Parser;
use enum_dispatch::enum_dispatch;

/// jwt command  命令
/// rCli jwt sign --sub akuowen --aud akuowen   --secret akuowen --exp (过期的秒时间戳) 应该直接添加的 暂时没加
/// 后续时间富裕可以添加其他jwt属性
/// see https://datatracker.ietf.org/doc/html/rfc7519#section-4.1
/// rCli jwt verify --token token  --secret akuowen
#[enum_dispatch(CmdExecutor)]
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
    #[arg(long, default_value_t = 14, help = "expiration time")]
    pub exp: i32,
    #[arg(long, help = "private key")]
    pub secret: String,
}

impl CmdExecutor for JwtSignOpt {
    async fn execute(self) -> anyhow::Result<()> {
        process_sign(&self.secret, &self.sub, &self.aud, self.exp).await
    }
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpt {
    #[arg(short, long, help = "token")]
    pub token: String,
    #[arg(long, help = "private key")]
    pub secret: String,
}

impl CmdExecutor for JwtVerifyOpt {
    async fn execute(self) -> anyhow::Result<()> {
        process_verify(&self.secret, &self.token).await
    }
}
