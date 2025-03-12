use axum::response::Html;
use std::env;

use crate::api;
use crate::models::TrainAnnouncement;
use crate::views;

pub async fn hello_world() -> Html<String> {
    let api_key = match env::var("TRAFIKVERKET_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return views::error_page("No API key found");
        }
    };

    match api::fetch_announcements(&api_key).await {
        Ok(api_response) => {
            let announcements: Vec<TrainAnnouncement> = api_response
                .response
                .result
                .into_iter()
                .flat_map(|r| r.train_announcements)
                .collect();

            views::render_train_table(announcements)
        }
        Err(e) => views::error_page(&format!("Error fetching data: {}", e)),
    }
}
