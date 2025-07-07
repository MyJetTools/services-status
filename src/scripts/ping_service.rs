use std::time::Duration;

use flurl::FlUrl;

use crate::{app_ctx::AppContext, models::ServiceApiIsAliveModel};

pub async fn ping_service(
    _app: &AppContext,
    unix_socket: String,
) -> Result<ServiceApiIsAliveModel, String> {
    let mut fl_url = FlUrl::new(unix_socket.as_str())
        .append_path_segment("api")
        .append_path_segment("isalive")
        .set_timeout(Duration::from_secs(3))
        .get()
        .await
        .unwrap();

    let body = fl_url.get_body_as_str().await.unwrap();

    let result: Result<ServiceApiIsAliveModel, serde_json::Error> = serde_json::from_str(body);
    match result {
        Ok(value) => Ok(value),
        Err(err) => Err(format!(
            "Can not deserialize response from {}. Err: {:?}",
            unix_socket, err
        )),
    }
}
