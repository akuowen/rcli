use crate::HttpFileOpts;
use anyhow::{Ok, Result};
use axum::{extract::Path, extract::State, http::StatusCode, response::Html, routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

async fn file_handler(
    State(state): State<Arc<FilePth>>,
    Path(path): Path<String>,
) -> (StatusCode, Html<String>) {
    // string::
    println!("===>{:?}", path);
    file_handler_self(State(state), Path(path)).await.unwrap()
}

struct FilePth {
    file_path: String,
}

pub async fn process_http_file(http_file_opts: &HttpFileOpts) -> Result<()> {
    let shared_state = Arc::new(FilePth {
        file_path: http_file_opts.file_path.clone(),
    });

    let app = Router::new()
        .nest_service("/static", ServeDir::new(http_file_opts.file_path.clone()))
        .route("/*path", get(file_handler))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", http_file_opts.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    println!(
        "server start at 0.0.0.0:{:?} , filepath: {:?}",
        http_file_opts.port, http_file_opts.file_path
    );
    Ok(())
}

// TODO 优化列表生成    优化静态文件读取路径 使用tower-http读取静态文件
async fn file_handler_self(
    State(state): State<Arc<FilePth>>,
    Path(path): Path<String>,
) -> Result<(StatusCode, Html<String>), anyhow::Error> {
    let context_path = &state.file_path;
    let p = std::path::Path::new(&context_path).join(&path);
    if !p.exists() {
        return Ok((StatusCode::NOT_FOUND, Html("file not found".to_string())));
    }

    let mut path_vec = Vec::new();
    let parent = format!("../{}", String::from(path.rsplitn(2, '/').last().unwrap()));

    path_vec.push(format!("<a href=\"{}\">..</a><br>", parent));
    if p.is_dir() {
        p.read_dir()?.try_for_each(|entry| {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if file_type.is_file() || file_type.is_dir() {
                let all_path = entry.path().as_path().to_str().unwrap().to_string();
                println!("all_path:{:?}", all_path);
                let path: String = if file_type.is_file() {
                    format!("../static{}", all_path.trim_start_matches('.'))
                } else {
                    format!(".{}", all_path)
                };
                println!("path: {:?}", path);
                let file_url = format!(
                    "<a href=\"{}\">{}</a><br>",
                    path,
                    entry.path().as_path().to_str().unwrap()
                );
                path_vec.push(file_url);
            }

            Ok(())
        })?;
        let result: String = path_vec.iter().fold(String::new(), |mut acc, fold| {
            acc.push_str(fold);
            acc.push('\n');
            acc
        });
        return Ok((StatusCode::OK, Html(format!("<html>{}</html>", result))));
    }

    let result = tokio::fs::read_to_string(p).await?;

    Ok((StatusCode::OK, Html(result)))
}
