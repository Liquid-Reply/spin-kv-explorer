use anyhow::Result;
use spin_sdk::{
    config,
    http::{Params, Request, Response},
};

use crate::models::ConValListModel;

pub(crate) fn handle_get_conf_vals(_req: Request, _params: Params) -> Result<Response> {
    let mut confvals: Vec<ConValListModel> = Vec::<ConValListModel>::default();

    let v_result = config::get("config_value");
    match v_result {
        Ok(v) => confvals.push(ConValListModel {
            key: "config_value".to_string(),
            value: v,
        }),
        Err(error) => panic!("Problem getting config_value: {:?}", error),
    };

    // TODO: implement config::get_all() in spin_sdk instead

    let body = serde_json::to_string(&confvals)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(body.into()))?)
}
