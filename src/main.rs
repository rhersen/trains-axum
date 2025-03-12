use axum::{Router, response::Html, routing::get};
use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::env;
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    #[serde(rename = "RESPONSE")]
    response: Response,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "RESULT")]
    result: Vec<Result>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    #[serde(rename = "TrainAnnouncement")]
    train_announcements: Vec<TrainAnnouncement>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrainAnnouncement {
    #[serde(rename = "AdvertisedTrainIdent")]
    advertised_train_ident: String,

    #[serde(rename = "AdvertisedTimeAtLocation")]
    advertised_time_at_location: DateTime<FixedOffset>,

    #[serde(rename = "ToLocation", default)]
    to_location: Vec<TrainLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrainLocation {
    #[serde(rename = "LocationName")]
    location_name: String,

    #[serde(rename = "Priority")]
    priority: i32,

    #[serde(rename = "Order")]
    order: i32,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        Router::new().route("/", get(hello_world)),
    )
    .await
    .unwrap();
}

async fn hello_world() -> Html<String> {
    let api_key = match env::var("TRAFIKVERKET_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Html(format!(
                r#"<html>
                    <head><title>Error</title></head>
                    <body><h1>Error: No API key found</h1></body>
                   </html>"#
            ));
        }
    };

    match req(xml_data(api_key)).send().await {
        Ok(response) => response_to_html(response).await,
        Err(e) => Html(format!(
            r#"<html>
                <head><title>Error</title></head>
                <body>
                    <h1>Error sending request</h1>
                    <p>{}</p>
                </body>
               </html>"#,
            e
        )),
    }
}

fn xml_data(api_key: String) -> String {
    return format!(
        r#"
<REQUEST>
  <LOGIN authenticationkey='{}' />
  <QUERY objecttype='TrainAnnouncement' sseurl='false' schemaversion='1.6'>
    <FILTER>
      <AND>
        <EQ name='LocationSignature' value='Sk' />
        <GT name='AdvertisedTimeAtLocation' value='2025-03-26T04:00Z' />
      </AND>
    </FILTER>
    <INCLUDE>AdvertisedTrainIdent</INCLUDE>
    <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
    <INCLUDE>ToLocation</INCLUDE>
  </QUERY>
</REQUEST>
"#,
        api_key
    );
}

fn req(xml_data: String) -> reqwest::RequestBuilder {
    Client::new()
        .post("https://api.trafikinfo.trafikverket.se/v2/data.json")
        .header("Content-Type", "application/xml")
        .body(xml_data)
}

async fn response_to_html(response: reqwest::Response) -> Html<String> {
    match response.text().await {
        Ok(text) => string_to_html(text),
        Err(e) => Html(format!(
            r#"<html>
                <head><title>Error</title></head>
                <body>
                    <h1>Error reading response</h1>
                    <p>{}</p>
                </body>
               </html>"#,
            e
        )),
    }
}

fn string_to_html(text: String) -> Html<String> {
    match serde_json::from_str::<ApiResponse>(&text) {
        Ok(parsed_response) => {
            let announcements: Vec<TrainAnnouncement> = parsed_response
                .response
                .result
                .into_iter()
                .flat_map(|r| r.train_announcements)
                .collect();

            announcements_to_html_table(announcements)
        }
        Err(e) => Html(format!(
            r#"<html>
                <head><title>Error</title></head>
                <body>
                    <h1>Failed to parse API response</h1>
                    <p>{}</p>
                    <pre>{}</pre>
                </body>
               </html>"#,
            e, text
        )),
    }
}

fn announcements_to_html_table(announcements: Vec<TrainAnnouncement>) -> Html<String> {
    // Sort announcements by time
    let mut sorted_announcements = announcements;
    sorted_announcements.sort_by_key(|a| a.advertised_time_at_location);

    // Build the HTML table
    let mut html = String::from(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Train Announcements</title>
            <style>
                body {
                    font-family: 'Segoe UI', Arial, sans-serif;
                    background-color: #121212;
                    color: #e0e0e0;
                    margin: 20px;
                    line-height: 1.6;
                }
                h1 {
                    color: #bb86fc;
                    margin-bottom: 20px;
                    border-bottom: 1px solid #333;
                    padding-bottom: 10px;
                }
                table {
                    border-collapse: collapse;
                    width: 100%;
                    margin-top: 25px;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
                }
                th, td {
                    padding: 15px;
                    text-align: left;
                    border-bottom: 1px solid #333;
                }
                th {
                    background-color: #1f1f1f;
                    color: #bb86fc;
                    font-weight: 600;
                }
                tr {
                    background-color: #1e1e1e;
                }
                tr:hover {
                    background-color: #2c~2c2c;
                }
                .time-column {
                    color: #03dac6; /* Teal accent for time */
                }
                .destination-column {
                    color: #cf6679; /* Pink-red for destination */
                }
                .train-column {
                    font-weight: 500;
                }
                @media (max-width: 600px) {
                    table, th, td {
                        font-size: 14px;
                        padding: 8px;
                    }
                    h1 {
                        font-size: 24px;
                    }
                }
            </style>
        </head>
        <body>
            <h1>Skövde Train Announcements</h1>
            <table>
                <tr>
                    <th>Train ID</th>
                    <th>Time</th>
                    <th>Destination</th>
                </tr>
        "#,
    );

    // Add rows for each announcement
    for announcement in sorted_announcements {
        // Format the datetime nicely
        let formatted_time = announcement
            .advertised_time_at_location
            .format("%H:%M, %d %b %Y");

        // Get destination (if any)
        let destination = if !announcement.to_location.is_empty() {
            // Find the location with highest priority (lowest number)
            announcement
                .to_location
                .iter()
                .min_by_key(|loc| loc.priority)
                .map_or("Unknown".to_string(), |loc| loc.location_name.clone())
        } else {
            "N/A".to_string()
        };

        html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>"#,
            announcement.advertised_train_ident, formatted_time, destination
        ));
    }

    // Close the HTML
    html.push_str(
        r#"
            </table>
        </body>
        </html>
        "#,
    );

    Html(html)
}
