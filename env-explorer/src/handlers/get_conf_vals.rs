use anyhow::Result;
use spin_sdk::{
    config,
    http::{Params, Request, Response
    },
};

use crate::models::ConValListModel;

pub(crate) fn handle_get_conf_vals(_req: Request, _params: Params) -> Result<Response> {
    let mut confvals: Vec<ConValListModel> = Vec::<ConValListModel>::default();

    // TODO: implement config::get_all() in spin_sdk instead
    confvals.push(ConValListModel{
        key: "environment".to_string(),
        value: config::get("environment").expect("Failed to acquire environment"),
    });

    let body = serde_json::to_string(&confvals)?;
    Ok(http::Response::builder()
    .status(http::StatusCode::OK)
    .body(Some(body.into()))?)
}
