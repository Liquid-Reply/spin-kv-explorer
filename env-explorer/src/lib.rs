mod entities;
mod handlers;
mod models;
use anyhow::Result;
use crate::handlers::{
    handle_get_ui,
    handle_get_list_keys,
    handle_get_key,
    handle_delete_key,
    handle_create_key,
    handle_get_env_vars,
    handle_get_conf_vals,
    handle_get_files,
};

use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_env_explorer(req: Request) -> Result<Response> {
    // // TODO: store credentials instead
    // let store = key_value::Store::open_default()?;
    // store.set("hello", "world").expect("Failed to set key");
    
    let mut router = Router::default();

    router.get("/", handle_get_ui);

	router.get("/api/stores/:store", handle_get_list_keys); // ListKeysHandler
	router.get("/api/stores/:store/keys/:key", handle_get_key); // GetKeyHandler
	router.delete("/api/stores/:store/keys/:key", handle_delete_key); // DeleteKeyHandler
	router.post("/api/stores/:store", handle_create_key); // AddKeyHandler

    router.get("/api/config/envvars", handle_get_env_vars);
    router.get("/api/config/confvals", handle_get_conf_vals);
    router.get("/api/config/files", handle_get_files);

    router.handle(req)
}
