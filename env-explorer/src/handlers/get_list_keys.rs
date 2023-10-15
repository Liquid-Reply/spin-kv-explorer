use anyhow::Result;
use spin_sdk::{
    http::{Params, Request, Response},
    key_value::Store,
};

use crate::models::AllKeysListModel;

pub(crate) fn handle_get_list_keys(_req: Request, _params: Params) -> Result<Response> {
    let Some(store_name) = _params.get("store") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    let store = Store::open(store_name)?;

    // start := time.Now()
    let Ok(keys) = store.get_keys() else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(None)?);
    };
    let mut result: AllKeysListModel = AllKeysListModel {
        store: store_name.to_string(),
        keys: Vec::<String>::default(),
    };
    keys.iter()
        .for_each(|item| result.keys.push(item.to_string()));
    // keys.iter()
    //     .flat_map(|key| store.get(key))
    //     .map(|entry| {
    //         let a = String::from_utf8(entry).expect("Found invalid UTF-8");
    //         a
    //     })
    //     .for_each(|item| {
    //         println!("key: {}", item);
    //         key_list.push(EntryListModel{ key: item })
    //     });

    let body = serde_json::to_string(&result)?;
    // log.Printf("LIST operation took: %s", time.Since(start))
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(body.into()))?)
}
