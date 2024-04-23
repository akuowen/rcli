mod cli;
mod process;
pub use cli::{B64Format, B64Ops, Commands, Format, GenPassOps, RCli, SubCommand};
pub use process::{process_base64, process_csv, process_passgen};
