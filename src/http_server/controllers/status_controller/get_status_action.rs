use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::models::ServicesStatusResponse;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/api/status",
    controller: "api",
    description: "Returns Services Status",
    summary: "Get Services Status",
)]
pub struct GetStatusAction {
    app: Arc<AppContext>,
}

impl GetStatusAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetStatusAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let contract = ServicesStatusResponse::new(&action.app).await;

    return HttpOutput::as_json(contract).into_ok_result(true).into();
}
