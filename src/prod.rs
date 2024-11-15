use std::sync::Arc;

use reqwest::Client;
use rerun::RecordingStream;

pub mod power;

const GLOBAL_TIME_CHAN: &str = "sim_time";

pub trait ProdLogger {
    fn from_client(
        client: Arc<Client>,
        rec: Arc<RecordingStream>,
        endpoint: impl Into<String>,
        channel: impl Into<String>,
    ) -> Self;
    async fn step(&self, time_step: f64);
}
