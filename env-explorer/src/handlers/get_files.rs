use anyhow::Result;
use spin_sdk::http::{Params, Request, Response};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::models::FileListModel;

pub(crate) fn handle_get_files(_req: Request, _params: Params) -> Result<Response> {
    let mut files: Vec<FileListModel> = Vec::<FileListModel>::default();

    let f = recurse_files("/");
    if let Ok(list) = f {
        list.into_iter().for_each(|file| {
            let fm = FileListModel {
                path: file.to_str().unwrap().to_string(),
            };
            files.push(fm);
            // println!("{:?}", file)
        });
    }

    let body = serde_json::to_string(&files)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(body.into()))?)
}

fn recurse_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = recurse_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
}
