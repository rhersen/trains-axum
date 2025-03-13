use crate::models::TrainAnnouncement;
use askama::Template;
use axum::response::Html;

// Template structs
#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

#[derive(Template)]
#[template(path = "train_table.html")]
struct TrainTableTemplate {
    announcements: Vec<AnnouncementView>,
}

// View model for train announcements
struct AnnouncementView {
    advertised_train_ident: String,
    formatted_time: String,
    destination: String,
}

pub fn error_page(message: &str) -> Html<String> {
    let template = ErrorTemplate {
        message: message.to_string(),
    };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
}

pub fn render_train_table(announcements: Vec<TrainAnnouncement>) -> Html<String> {
    let announcement_views: Vec<AnnouncementView> = announcements
        .into_iter()
        .map(|announcement| {
            // Format the datetime nicely
            let formatted_time = announcement
                .advertised_time_at_location
                .format("%H:%M:%S")
                .to_string();

            // Get destination (if any)
            let destination = if !announcement.to_location.is_empty() {
                announcement
                    .to_location
                    .iter()
                    .min_by_key(|loc| loc.priority)
                    .map_or("Unknown".to_string(), |loc| loc.location_name.clone())
            } else {
                "N/A".to_string()
            };

            AnnouncementView {
                advertised_train_ident: announcement.advertised_train_ident,
                formatted_time,
                destination,
            }
        })
        .collect();

    let template = TrainTableTemplate {
        announcements: announcement_views,
    };

    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
}
