use std::time::Duration;

use flurl::FlUrl;

use crate::{app_ctx::AppContext, models::ServiceApiIsAliveModel};

pub async fn ping_service(
    _app: &AppContext,
    unix_socket: String,
) -> Result<ServiceApiIsAliveModel, String> {
    let fl_url_result = FlUrl::new(unix_socket.as_str())
        .append_path_segment("api")
        .append_path_segment("isalive")
        .set_timeout(Duration::from_secs(3))
        .get()
        .await;

    let mut fl_url_result = match fl_url_result {
        Ok(fl_url_result) => fl_url_result,
        Err(err) => {
            return Err(format!("{:?}", err));
        }
    };

    let body = match fl_url_result.get_body_as_str().await {
        Ok(body) => body,
        Err(err) => {
            return Err(format!("FlUrlGetBody err:{:?}", err));
        }
    };

    let result: Result<ServiceApiIsAliveModel, serde_json::Error> = serde_json::from_str(body);
    match result {
        Ok(value) => Ok(value),
        Err(err) => Err(format!(
            "Can not deserialize response from {}. Err: {:?}",
            unix_socket, err
        )),
    }
}
