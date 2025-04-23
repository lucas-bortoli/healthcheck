use chrono::Local;
use json5;
use notify_rust::Notification;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{fs, io::Write, path::PathBuf, thread, time::Duration};

#[derive(Debug, Deserialize)]
struct Endpoint {
    url: String,
    interval: u64,       // seconds
    expect: Option<u16>, // optional expected HTTP status code
}

type Config = Vec<Endpoint>;

const CONFIG_FILE: &str = "healthcheck_config.json5";
const LOG_FILE: &str = "healthcheck_log.log";

fn main() {
    // get the current exe path
    let exe_path = std::env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    let config_path = exe_dir.join(CONFIG_FILE);

    // create sample config and exit if none exists
    if !config_path.exists() {
        create_sample_config(&config_path).expect("Failed to create sample config");
        println!(
            "Sample config created at {:?}! Please edit it and rerun.",
            config_path
        );
        return;
    }

    // parse config
    let config_str = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: Config = json5::from_str(&config_str).expect("Failed to parse config file");

    println!("Starting healthcheck with config: {:#?}", config);

    // create HTTP client with timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client");

    // spawn a thread for each endpoint so their timings are independent
    let mut handles = vec![];
    for endpoint in config {
        let client = client.clone();
        let handle = thread::spawn(move || {
            // immediate first check
            check_endpoint(&client, &endpoint);

            // ...then check at regular intervals
            loop {
                thread::sleep(Duration::from_secs(endpoint.interval));
                check_endpoint(&client, &endpoint);
            }
        });
        handles.push(handle);
    }

    // wait for all threads - though they should run indefinitely
    for handle in handles {
        handle.join().unwrap();
    }
}

fn create_sample_config(path: &PathBuf) -> std::io::Result<()> {
    let sample = r#"
[
  {
    url: "https://example.com",
    interval: 60,
  },
  {
    url: "https://example.com/fake_page_should_404",
    interval: 600, // 10 minutes
    expect: 404, // this URL should return 404 Not Found
  },
]
"#;
    let mut file = fs::File::create(path)?;
    file.write_all(sample.as_bytes())?;
    Ok(())
}

fn log_message(message: &str) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Failed to open log file");
    writeln!(file, "{}", message).expect("Failed to write to log file");
}

fn check_endpoint(client: &Client, endpoint: &Endpoint) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    match client.get(&endpoint.url).send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let expected = endpoint.expect.unwrap_or(200);
            if status == expected {
                let msg = format!("[{}] {} - OK (Status: {})", now, endpoint.url, status);
                println!("{}", msg);
                log_message(&msg);
            } else {
                let msg = format!(
                    "[{}] {} - ERROR: Expected {}, got {}",
                    now, endpoint.url, expected, status
                );
                println!("{}", msg);
                log_message(&msg);
                notify(
                    &endpoint.url,
                    &format!("Expected {}, got {}", expected, status),
                );
            }
        }
        Err(err) => {
            let msg = format!("[{}] {} - ERROR: {}", now, endpoint.url, err);
            println!("{}", msg);
            log_message(&msg);
            notify(&endpoint.url, &err.to_string());
        }
    }
}

fn notify(url: &str, error: &str) {
    Notification::new()
        .summary(&format!("Healthcheck Failed: {}", url))
        .body(error)
        .show()
        .expect("Failed to show notification");
}
