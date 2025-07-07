use flurl::FlUrl;
use rust_extensions::StopWatch;

use crate::app_ctx::AppContext;

pub async fn ping_service(
    _app: &AppContext,
    unix_socket: String,
) -> Result<PingServiceResult, String> {
    let sw = StopWatch::new();
    let mut fl_url = FlUrl::new(unix_socket)
        .append_path_segment("api")
        .append_path_segment("isalive")
        .get()
        .await
        .unwrap();

    let duration = sw.duration();

    let body = fl_url.get_body_as_str().await.unwrap();

    println!("[{:?}] {}", duration, body);

    Ok(PingServiceResult {})
}

pub struct PingServiceResult {}
