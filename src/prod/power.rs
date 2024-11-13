use std::{sync::Arc, time};

use reqwest::Client;
use rerun::RecordingStream;

use super::ProdLogger;

pub struct PowerProdLogger {
    client: Arc<Client>,
    rec: Arc<RecordingStream>,
    endpoint: String,
    channel: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct PowerProdData {
    battery_capacity: f64,
    power_consumed: f64,
}

impl ProdLogger for PowerProdLogger {
    fn from_client(
        client: Arc<Client>,
        rec: Arc<RecordingStream>,
        endpoint: impl Into<String>,
        channel: impl Into<String>,
    ) -> Self {
        let logger = PowerProdLogger {
            client,
            rec: rec.clone(),
            endpoint: endpoint.into(),
            channel: channel.into(),
        };

        rec.log_static(
            logger.channel.clone(),
            &rerun::SeriesLine::new()
                .with_color([255, 0, 0])
                .with_name("Power Consumption"),
        )
        .unwrap();

        logger
    }

    async fn step(&self, time_step: f64) {
        let resp: Vec<PowerProdData> = self
            .client
            .get(&self.endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        self.rec.set_time_seconds("sim_time", time_step);

        self.rec
            .clone()
            .log(
                self.channel.clone(),
                &rerun::Scalar::new(resp[0].power_consumed),
            )
            .unwrap();
    }
}
