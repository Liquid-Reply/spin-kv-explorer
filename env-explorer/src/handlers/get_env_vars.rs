use anyhow::Result;
use spin_sdk::http::{Params, Request, Response,
};
use std::env;

use crate::models::EnvVarListModel;

pub(crate) fn handle_get_env_vars(_req: Request, _params: Params) -> Result<Response> {
    let mut envvars: Vec<EnvVarListModel> = Vec::<EnvVarListModel>::default();

    env::vars()
        .for_each(|(k, v)| {
            // println!("{}: {}", &k, &v);
            envvars.push(EnvVarListModel { key: k, value: v });
        });

    let body = serde_json::to_string(&envvars)?;
    Ok(http::Response::builder()
    .status(http::StatusCode::OK)
    .body(Some(body.into()))?)
}
