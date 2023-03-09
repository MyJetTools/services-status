use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType};

use crate::app_ctx::AppContext;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/",
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
    let content = format!(
        r###"<html><head><title>{ver} Services Status</title>
        <link rel="icon" type="image/png" href="/img/favicon.png" />
        <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
        <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
        <link href="/css/site.css" rel="stylesheet" type="text/css" />
        <script src="/js/jquery.js"></script><script src="/js/app.js?ver={rnd}"></script>
        </head><body></body></html>"###,
        ver = crate::app_ctx::APP_VERSION,
        rnd = action.app.process_id
    );

    HttpOutput::Content {
        headers: None,
        content_type: Some(WebContentType::Html),
        content: content.into_bytes(),
    }
    .into_ok_result(true)
    .into()
}
