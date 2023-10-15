use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value::Store,
};

use crate::models::EntryModel;

pub(crate) fn handle_get_key(_req: Request, _params: Params) -> Result<Response> {
    let Some(key) = _params.get("key") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    let Some(store_name) = _params.get("store") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };
    let store = Store::open(store_name)?;

    // start := time.Now()
    let Ok(found) = store.get(key) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    let value = String::from_utf8(found).expect("Found invalid UTF-8");

    let model = EntryModel {
        store: store_name.to_string(),
        key: key.to_string(),
        value,
    };
    let serialized = serde_json::to_string(&model)?;
    // log.Printf("LIST operation took: %s", time.Since(start))
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(serialized.into()))?)
}
