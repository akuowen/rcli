mod cli;
mod process;
pub use cli::{RCli, SubCommand};
pub use process::{process_csv, process_passgen};
