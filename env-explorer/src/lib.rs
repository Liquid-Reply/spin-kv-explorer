mod entities;
mod handlers;
mod models;
use random_string::generate;

use anyhow::Result;
use http_auth_basic::Credentials;
use std::env;
use crate::handlers::{
    handle_create_key, handle_delete_key, handle_get_conf_vals, handle_get_env_vars,
    handle_get_files, handle_get_key, handle_get_list_keys, handle_get_ui,
};

use spin_sdk::{
    key_value::Store,
    http::{Params, Request, Response, Router},
    http_component,
};

const KV_STORE_CREDENTIALS_KEY: &str = "kv-credentials";
const SKIP_AUTH_ENV: &str = "SPIN_APP_KV_SKIP_AUTH";

/// A simple Spin HTTP component.
#[http_component]
fn handle_env_explorer(req: Request) -> Result<Response> {
    // // TODO: store credentials instead
    // let store = key_value::Store::open_default()?;
    // store.set("hello", "world").expect("Failed to set key");

    let creds = get_credentials()?;

    let mut router = Router::default();

    
    router.get("/", handle_get_ui);
    // router.get("/", |req, params| is_authenticated(req, params, &handle_get_ui));
    router.get("/api/stores/:store", |req, params| {
        is_authenticated(req, params, &handle_get_list_keys)
    }); // ListKeysHandler
    router.get("/api/stores/:store/keys/:key", |req, params| {
        is_authenticated(req, params, &handle_get_key)
    }); // GetKeyHandler
    router.delete("/api/stores/:store/keys/:key", |req, params| {
        is_authenticated(req, params, &handle_delete_key)
    }); // DeleteKeyHandler
    router.post("/api/stores/:store", |req, params| {
        is_authenticated(req, params, &handle_create_key)
    }); // AddKeyHandler
        // TODO: add update key handler

    router.get("/api/config/envvars", |req, params| {
        is_authenticated(req, params, &handle_get_env_vars)
    });
    router.get("/api/config/confvals", |req, params| {
        is_authenticated(req, params, &handle_get_conf_vals)
    });
    router.get("/api/config/files", |req, params| {
        is_authenticated(req, params, &handle_get_files)
    });

    router.handle(req)
}

fn is_authenticated(
    r: Request,
    params: Params,
    handler: &dyn Fn(Request, Params) -> Result<Response>,
) -> Result<Response> {

    let val = env::var(SKIP_AUTH_ENV).unwrap_or("0".to_string());
    println!("SKIP_AUTH_ENV: {}", val);
    if val == "1" {
        return handler(r, params);
    }

    let Some(header_value) = r.headers().get("Authorization") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::FORBIDDEN)
            .body(Some("No token presented".into()))?);
    };

    let value = header_value.to_str()?;
    match Credentials::from_header(value.to_string()) {
        Ok(creds) => {
            if creds.user_id == "admin" {
                return handler(r, params);
            }
            Ok(http::Response::builder()
                .status(http::StatusCode::FORBIDDEN)
                .body(Some("Wrong user".into()))?)
        }
        Err(_) => Ok(http::Response::builder()
            .status(http::StatusCode::FORBIDDEN)
            .body(Some("Error decoding header value".into()))?),
    }
}

fn get_credentials() -> Result<Credentials> {
    let store = Store::open_default()?;
    
    let exists = store.exists(KV_STORE_CREDENTIALS_KEY)?;
    if !exists {
        // generate defaultUser
        let default_user = generate_random_string(10);
        let default_password = generate_random_string(30);

        store.set(&KV_STORE_CREDENTIALS_KEY, format!("{default_user}:{default_password}"))?;

        let creds = Credentials{
            user_id: default_user.to_string(),
            password: default_password.to_string(),
        };

        return Ok(creds)
    }

    let creds = String::from_utf8(store.get(KV_STORE_CREDENTIALS_KEY)?)?;
    let mut split = creds.split(':');

    let default_user = split.next().unwrap();
    let default_password = split.next().unwrap();

    let creds = Credentials::new(&default_user, &default_password);

    return Ok(creds);
}

fn generate_random_string(n: usize) -> String {
    let charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-!@#$%^&*";
    generate(n, charset)
}
