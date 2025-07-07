use std::{sync::Arc, time::Duration};

use flurl::FlUrlResponse;

use serde_derive::{Deserialize, Serialize};

use rust_extensions::{MyTimerTick, StopWatch};

use crate::{app_ctx::AppContext, settings::ServiceSettings};

pub struct ServicesPinger {
    app: Arc<AppContext>,
}

impl ServicesPinger {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for ServicesPinger {
    async fn tick(&self) {
        /*
        let settings_model = self.app.settings_reader.get_settings().await;

        self.app
            .services_list
            .sync_services_list(&settings_model.services)
            .await;

        for (domain, services) in &settings_model.services {
            for service in services {
                let app = self.app.clone();
                let domain = domain.clone();
                let service = service.clone();
                tokio::spawn(ping_service(app, domain, service));
            }
        }
         */

        let (path, sockets) = crate::scripts::get_sockets_list(&self.app).await;

        for socket in sockets {
            let mut unix_socket = path.clone();
            unix_socket.append_segment(&socket);

            let unix_socket = format!("http+unix:/{}", unix_socket.as_str());

            let result = crate::scripts::ping_service(&self.app, unix_socket).await;
        }
    }
}

async fn get_ok_result(mut result: FlUrlResponse) -> Result<ServiceApiIsAliveModel, String> {
    if result.get_status_code() != 200 {
        return Err(format!(
            "Http Status code is not 200. It is {}",
            result.get_status_code()
        ));
    }

    let get_body_result = result.get_body_as_slice().await;

    if let Err(err) = get_body_result {
        return Err(format!("Can not get body: {:?}", err));
    }

    let body = get_body_result.unwrap();

    let model = serde_json::from_slice::<ServiceApiIsAliveModel>(body);

    //cSpell: disable
    if let Err(err) = model {
        return Err(format!(
            "Can not deserialize into api/isalive model: {:?}",
            err
        ));
    }
    //cSpell: enable

    Ok(model.unwrap())
}

async fn ping_service(app: Arc<AppContext>, domain: String, service: ServiceSettings) {
    let mut sw = StopWatch::new();
    sw.start();
    let result = flurl::FlUrl::new(service.url.as_str())
        .set_timeout(Duration::from_secs(2))
        .get()
        .await;

    match result {
        Ok(response) => match get_ok_result(response).await {
            Ok(model) => {
                sw.pause();
                app.services_list
                    .update_ok(
                        domain.as_str(),
                        service.id.as_str(),
                        model.name,
                        model.version,
                        model.compiled,
                        model.env_info,
                        model.started,
                        sw.duration(),
                    )
                    .await;
            }
            Err(err) => {
                sw.pause();
                app.services_list
                    .update_error(domain.as_str(), service.id.as_str(), err, sw.duration())
                    .await;
            }
        },
        Err(err) => {
            sw.pause();
            app.services_list
                .update_error(
                    domain.as_str(),
                    service.id.as_str(),
                    format!("{:?}", err),
                    sw.duration(),
                )
                .await;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceApiIsAliveModel {
    pub name: String,
    pub version: String,
    pub env_info: Option<String>,
    pub compiled: Option<String>,
    pub started: Option<i64>,
}
