use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct DnsRecordUpdate {
    comment: String,
    name: String,
    proxied: bool,
    tags: Vec<String>,
    ttl: u32,
    content: String,
    #[serde(rename = "type")]
    record_type: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    success: bool,
    errors: Vec<ApiError>,
    result: Option<DnsRecordResult>,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    message: String,
}

#[derive(Deserialize, Debug)]
struct DnsRecordResult {
    name: String,
    content: String,
}

async fn get_external_ip() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(response)
}

async fn update_dns_record(client: &Client, zone_id: &str, record_id: &str, new_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("CLOUDFLARE_API_TOKEN")?;
    let dns_record = DnsRecordUpdate {
        comment: "DDNS update".to_string(),
        name: env::var("DNS_RECORD_NAME")?,
        proxied: true,
        tags: vec![],
        ttl: 3600,
        content: new_ip.to_string(),
        record_type: "A".to_string(),
    };

    let response: ApiResponse = client
        .put(&format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, record_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&dns_record)
        .send()
        .await?
        .json()
        .await?;

    if response.success {
        if let Some(result) = response.result {
            println!("DNS record updated successfully: {} -> {}", result.name, result.content);
        }
    } else {
        eprintln!("Failed to update DNS record: {:?}", response.errors);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let client = Client::new();

    let zone_id = env::var("CLOUDFLARE_ZONE_ID")?;
    let record_id = env::var("CLOUDFLARE_RECORD_ID")?;

    let mut last_ip = String::new();

    loop {
        match get_external_ip().await {
            Ok(current_ip) => {
                if current_ip != last_ip {
                    println!("IP change detected: {} -> {}", last_ip, current_ip);
                    if let Err(e) = update_dns_record(&client, &zone_id, &record_id, &current_ip).await {
                        eprintln!("Error updating DNS record: {}", e);
                    } else {
                        last_ip = current_ip;
                    }
                } else {
                    println!("IP has not changed. Current IP: {}", current_ip);
                }
            }
            Err(e) => eprintln!("Error fetching external IP: {}", e),
        }
        sleep(Duration::from_secs(60)).await;
    }
}
