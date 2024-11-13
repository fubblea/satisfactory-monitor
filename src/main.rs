use std::{sync::Arc, thread, time};

use futures::future::join_all;
use prod::{power::PowerProdLogger, ProdLogger};
use reqwest::Client;

mod prod;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Polling Rate [Hz]
    #[arg(short, long, default_value_t = 2.0)]
    rate: f64,
}

#[tokio::main]
async fn main() {
    // Parse CLI args
    let args = Args::parse();

    // Swap Rerun Viewer
    let opts = rerun::SpawnOptions::default();
    let _ = rerun::spawn(&opts);

    // Make a connection to the Rerun instance
    let rec = Arc::new(
        rerun::RecordingStreamBuilder::new("satisfactory_monitor")
            .connect()
            .unwrap(),
    );

    // Init client connection
    const SERVER_URL: &str = "http://localhost:8080";
    let client = Arc::new(Client::new());

    // Init loggers
    let power_logger = PowerProdLogger::from_client(
        client.clone(),
        rec.clone(),
        format!("{}/getPower", SERVER_URL),
        "prod/power",
    );

    // Logging loop
    let start_time = time::SystemTime::now();
    loop {
        thread::sleep(time::Duration::from_millis(
            (1 / (args.rate.round() as u64)) * 100,
        ));

        // Poll all tasks
        let tasks = vec![power_logger.step(start_time.elapsed().unwrap().as_secs_f64())];

        // Await the completions of all logging tasks
        let result = join_all(tasks).await;
    }
}
