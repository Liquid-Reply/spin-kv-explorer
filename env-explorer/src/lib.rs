mod entities;
mod handlers;
mod models;
use anyhow::Result;
// use http_auth_basic::Credentials;
// use std::env;
use crate::handlers::{
    handle_create_key, handle_delete_key, handle_get_conf_vals, handle_get_env_vars,
    handle_get_files, handle_get_key, handle_get_list_keys, handle_get_ui,
};

use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};

// const KV_STORE_CREDENTIALS_KEY: &str = "kv-credentials";
// const SKIP_AUTH_ENV: &str = "SPIN_APP_KV_SKIP_AUTH";

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
                                                          // TODO: add update key handler

    router.get("/api/config/envvars", handle_get_env_vars);
    router.get("/api/config/confvals", handle_get_conf_vals);
    router.get("/api/config/files", handle_get_files);

    router.handle(req)
}

// fn basic_auth(req: Request, required_user: &str, required_password: &str) -> WTFDOIRETURN? {

//     let mut router = Router::default();

//     let val = env::var(SKIP_AUTH_ENV).unwrap();
//     if val == "1" {
//         return req;
//     }

//     let auth_header_value = req.headers().get("Authorization").unwrap();
//     let auth_header_value_string = auth_header_value.to_str().unwrap();
//     println!("auth_header_value: {:?}", auth_header_value);
//     // let auth_header_value = String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
//     let credentials = Credentials::from_header(auth_header_value_string.into()).unwrap();

//     if credentials.user_id == required_user && credentials.password == required_password {
//         Ok(req)
//     } else {
//         http::Response::builder()
//         .status(http::StatusCode::UNAUTHORIZED)
//         .header("WWW-Authenticate", "Basic realm=Restricted")
//         .body::(None)?;

//         Ok(req)
//     }
// }
