use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value::Store,
};

use crate::entities::Entry;
use crate::models::CreateEntryModel;

pub(crate) fn handle_create_key(_req: Request, _params: Params) -> Result<Response> {
    // what if key existing?
    // let Some(key) = _params.get("key") else {
    //     return Ok(http::Response::builder()
    //         .status(http::StatusCode::NOT_FOUND)
    //         .body(None)?);
    // };
    // start := time.Now()

    let Ok(model) = CreateEntryModel::try_from(&_req.body().clone()) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .body(None)?);
    };

    let Some(store_name) = _params.get("store") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };
    let store = Store::open(store_name)?;

    let entry = Entry {
        store: model.store,
        key: model.key,
        value: model.value,
    };

    let serialized = serde_json::to_vec(&entry)?;
    store.set(&entry.key, &entry.value)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        .body(Some(serialized.into()))?)
}
