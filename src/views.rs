use crate::locations::name;
use crate::models::TrainAnnouncement;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "station.html")]
struct StationTemplate {
    location_name: String,
    announcements: Vec<AnnouncementView>,
}

#[derive(Template)]
#[template(path = "train.html")]
struct TrainTemplate {
    id: String,
    destination: String,
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

fn advertised_time(announcement: &TrainAnnouncement) -> String {
    announcement
        .advertised_time_at_location
        .format("%H:%M")
        .to_string()
}

fn actual_time(announcement: &TrainAnnouncement) -> String {
    announcement
        .time_at_location_with_seconds
        .map_or("".to_string(), |time| time.format("%H:%M:%S").to_string())
}

fn destination(announcement: &TrainAnnouncement) -> String {
    if !announcement.to_location.is_empty() {
        announcement
            .to_location
            .iter()
            .min_by_key(|loc| loc.priority)
            .map_or("Unknown".to_string(), |loc| name(&loc.location_name))
    } else {
        "N/A".to_string()
    }
}

fn train_ident(announcements: &Vec<AnnouncementView>) -> String {
    announcements
        .first()
        .map(|a| a.advertised_train_ident.clone())
        .unwrap_or("Unknown".to_string())
}

fn dest(announcements: &Vec<AnnouncementView>) -> String {
    announcements
        .first()
        .map(|a| a.destination.clone())
        .unwrap_or("Unknown".to_string())
}

pub fn render_station(announcements: Vec<TrainAnnouncement>) -> Html<String> {
    let announcement_views: Vec<AnnouncementView> = announcements
        .into_iter()
        .map(|announcement| {
            let advertised_time = advertised_time(&announcement);
            let actual_time = actual_time(&announcement);
            let location_signature = announcement.location_signature.clone();
            let location_name = name(&announcement.location_signature);
            let destination = destination(&announcement);

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

    let location_name = announcement_views
        .first()
        .map(|a| a.location_signature.as_str())
        .unwrap_or("Unknown");

    let template = StationTemplate {
        location_name: name(&location_name),
        announcements: announcement_views,
    };

    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
}

pub fn render_train(announcements: Vec<TrainAnnouncement>) -> Html<String> {
    let announcement_views: Vec<AnnouncementView> = announcements
        .into_iter()
        .map(|announcement| {
            let advertised_time = advertised_time(&announcement);
            let actual_time = actual_time(&announcement);
            let location_signature = announcement.location_signature.clone();
            let location_name = name(&announcement.location_signature);
            let destination = destination(&announcement);

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

    let template = TrainTemplate {
        id: train_ident(&announcement_views),
        destination: dest(&announcement_views),
        announcements: announcement_views,
    };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
}
