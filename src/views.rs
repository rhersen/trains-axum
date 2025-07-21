use crate::locations::name;
use crate::models::TrainAnnouncement;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "station.html")]
struct StationTemplate {
    announcements: Vec<AnnouncementView>,
}

struct AnnouncementView {
    advertised_train_ident: String,
    advertised_time: String,
    actual_time: String,
    destination: String,
    location_signature: String,
    location_name: String,
}

pub fn render_station(announcements: Vec<TrainAnnouncement>) -> Html<String> {
    let announcement_views: Vec<AnnouncementView> = announcements
        .into_iter()
        .map(|announcement| {
            let advertised_time = announcement
                .advertised_time_at_location
                .format("%H:%M")
                .to_string();
            let actual_time = announcement
                .time_at_location_with_seconds
                .map_or("".to_string(), |time| time.format("%H:%M:%S").to_string());
            let location_signature = announcement.location_signature.clone();
            let location_name = name(announcement.location_signature);
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
                advertised_time,
                actual_time,
                destination,
                location_signature,
                location_name,
            }
        })
        .collect();

    let template = StationTemplate {
        announcements: announcement_views,
    };

    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
}
