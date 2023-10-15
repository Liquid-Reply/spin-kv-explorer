use anyhow::Result;
use askama::Template;
use spin_sdk::http::{Params, Request, Response};

const SPIN_ROUTE: &str = "";

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct UiTemplate<'a> {
    // the name of the struct can be anything
    spinroute: &'a str, // the field name should match the variable name
                        // in your template
}

pub(crate) fn handle_get_ui(_req: Request, _params: Params) -> Result<Response> {
    let ui = UiTemplate {
        spinroute: SPIN_ROUTE,
    }; // instantiate your struct
    let html = ui.render().unwrap();

    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(html.into()))?)
}
