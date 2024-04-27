use crate::{process_http_file, CmdExecutor};
use clap::Parser;

/// http file server
/// 支持文件夹 / 文件展示  tower-http service展示静态文件 axum 负责文件夹的递归
#[derive(Debug, Parser)]
pub struct HttpFileOpts {
    #[arg(short, long, default_value = "8080", help = "port number")]
    pub port: u16,
    #[arg(short, long, default_value = ".", help = "file path")]
    pub file_path: String,
}

impl CmdExecutor for HttpFileOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_file(&self).await
    }
}
