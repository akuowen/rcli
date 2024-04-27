mod cli;
mod process;
mod util;

use crate::cli::SubCommand;
pub use cli::{
    B64DecodeOps, B64EncodeOps, B64Format, B64Ops, Commands, Format, GenPassOps, HttpFileOpts,
    JwtOpts, JwtSignOpt, JwtVerifyOpt, RCli, SignOpts, TextFormat, TextOps, TextSignOpts,
    TextSignVerifyOpt, TextVerifyOpts, VerifyOpts,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    process_base64, process_csv, process_decode, process_encode, process_http_file, process_jwt,
    process_passgen, process_sign, process_text, process_text_sign, process_text_verify,
    process_verify, sign, verify,
};
pub use util::{get_content, get_reader};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
