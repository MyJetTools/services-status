use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType};

use crate::app_ctx::AppContext;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/metrics",
)]
pub struct IndexAction {
    pub app: Arc<AppContext>,
}

impl IndexAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &IndexAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut result = String::new();

    result.push_str("# HELP dead status\n");

    for i in 0..10 {
        result.push_str(&format!(""));
    }
}
