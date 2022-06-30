extern crate lazy_static;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::env;

lazy_static::lazy_static! {
    static ref H1_AUTH: ApiCredentials = parse_credentials();
}

struct ApiCredentials {
    username: String,
    key: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub data: ReportData,
}

#[derive(Deserialize, Debug)]
pub struct ReportData {
    pub id: String,
    #[serde(alias = "type")]
    pub report_type: String,
    pub attributes: ReportAttributes,
    pub relationships: Relationships,
}

#[derive(Deserialize, Debug)]
pub struct ReportAttributes {
    pub title: String,
    pub state: String,
    pub created_at: String,
    pub triaged_at: Option<String>,
    pub last_reporter_activity_at: Option<String>,
    pub last_program_activity_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Relationships {
    pub severity: Option<Severity>,
}

#[derive(Deserialize, Debug)]
pub struct Severity {
    pub data: SeverityData,
}

#[derive(Deserialize, Debug)]
pub struct SeverityData {
    pub id: String,
    #[serde(alias = "type")]
    pub severity_type: String,
    pub attributes: SeverityAttributes,
}

#[derive(Deserialize, Debug)]
pub struct SeverityAttributes {
    pub rating: Option<String>,
    pub score: Option<f32>,
}

fn parse_credentials() -> ApiCredentials {
    ApiCredentials {
        username: env::var("H1_USERNAME").expect("expected H1_USERNAME"),
        key: env::var("H1_APIKEY").expect("expected H1_APIKEY"),
    }
}

pub async fn get_report(report_id: &str) -> Result<ApiResponse> {
    let client = reqwest::Client::new();
    let request_url = format!("https://api.hackerone.com/v1/reports/{report_id}");
    let response = client
        .get(request_url)
        .basic_auth(&H1_AUTH.username, Some(&H1_AUTH.key))
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<ApiResponse>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow!(e)),
        },
        reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Unauthorized")),
        other => Err(anyhow!(other)),
    }
}

pub async fn get_bounty_info(report_id: &str, username: &str, api_key: &str) {}
