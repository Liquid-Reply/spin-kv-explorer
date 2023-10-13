use anyhow::Result;
use spin_sdk::{
    key_value::Store,
    http::{Params, Request, Response,},
};

pub(crate) fn handle_delete_key(_req: Request, _params: Params) -> Result<Response> {
    // start := time.Now()
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

    let Ok(exists) = store.exists(key) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    if !exists {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    }

    let Ok(_) = store.delete(key) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(None)?);
    };

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(None)?)
}
