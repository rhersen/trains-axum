use axum::response::Html;

use crate::api;
use crate::models::TrainAnnouncement;
use crate::views;

pub async fn stations() -> axum::response::Html<String> {
    return Html("<a href='station/Sk'>Sk</a><br/><a href='station/Tul'>Tul</a>".to_string());
}

pub async fn station(
    axum::extract::Path(code): axum::extract::Path<String>,
) -> axum::response::Html<String> {
    let api_key = match std::env::var("TRAFIKVERKET_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Html("No API key found".to_string());
        }
    };

    match api::fetch_announcements(&api_key, &code).await {
        Ok(api_response) => {
            let announcements: Vec<TrainAnnouncement> = api_response
                .response
                .result
                .into_iter()
                .flat_map(|r| r.train_announcements)
                .collect();

            views::render_station(announcements)
        }
        Err(e) => Html(format!("Error fetching data: {}", e)),
    }
}
