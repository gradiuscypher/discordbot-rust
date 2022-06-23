use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: ReportData,
}

#[derive(Deserialize, Debug)]
struct ReportData {
    id: String,
    #[serde(alias = "type")]
    report_type: String,
    attributes: ReportAttributes,
    relationships: Relationships,
}

#[derive(Deserialize, Debug)]
struct ReportAttributes {
    title: String,
    state: String,
    created_at: String,
    triaged_at: Option<String>,
    last_reporter_activity_at: Option<String>,
    last_program_activity_at: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Relationships {
    severity: Option<Severity>,
}

#[derive(Deserialize, Debug)]
struct Severity {
    data: SeverityData,
}

#[derive(Deserialize, Debug)]
struct SeverityData {
    id: String,
    #[serde(alias = "type")]
    severity_type: String,
    attributes: SeverityAttributes,
}

#[derive(Deserialize, Debug)]
struct SeverityAttributes {
    rating: Option<String>,
    score: Option<f32>,
}

pub async fn get_report(report_id: &str, username: &str, api_key: &str) {
    let client = reqwest::Client::new();
    let request_url = format!("https://api.hackerone.com/v1/reports/{report_id}");
    let response = client
        .get(request_url)
        .basic_auth(username, Some(api_key))
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<ApiResponse>().await {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => println!("unable to parse json: {}", e),
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("auth failed")
        }
        other => {
            panic!("Something bad happened: {:?}", other)
        }
    }
}
