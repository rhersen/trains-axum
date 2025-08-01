use crate::locations::name;
use crate::models::{TrainAnnouncement, TrainLocation};
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
    from: String,
    via: String,
    destination: String,
    product_information: String,
    announcements: Vec<AnnouncementView>,
}

struct AnnouncementView {
    advertised_train_ident: String,
    advertised_time: String,
    actual_time: String,
    destination: String,
    from: String,
    location_signature: String,
    location_name: String,
    product_information: String,
}

pub fn render_station(announcements: &Vec<TrainAnnouncement>) -> Html<String> {
    let announcement_views: Vec<AnnouncementView> = announcements
        .into_iter()
        .map(|announcement| AnnouncementView {
            advertised_train_ident: announcement.advertised_train_ident.clone(),
            location_signature: announcement.location_signature.clone(),
            advertised_time: advertised_time(&announcement),
            actual_time: actual_time(&announcement),
            from: origin(&announcement),
            destination: destination(&announcement),
            location_name: name(&announcement.location_signature),
            product_information: product_information(announcement),
        })
        .collect();

    let template = StationTemplate {
        location_name: location(announcements),
        announcements: announcement_views,
    };

    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
}

pub fn render_train(announcements: &Vec<TrainAnnouncement>) -> Html<String> {
    let announcement_views: Vec<AnnouncementView> = announcements
        .into_iter()
        .map(|announcement| AnnouncementView {
            advertised_train_ident: announcement.advertised_train_ident.clone(),
            location_signature: announcement.location_signature.clone(),
            advertised_time: advertised_time(&announcement),
            actual_time: actual_time(&announcement),
            from: origin(&announcement),
            destination: destination(&announcement),
            location_name: name(&announcement.location_signature),
            product_information: product_information(announcement),
        })
        .collect();

    let template = TrainTemplate {
        id: train_ident(announcements),
        from: from(announcements),
        via: via_stations(announcements),
        destination: dest(announcements),
        product_information: prod(announcements),
        announcements: announcement_views,
    };
    Html(
        template
            .render()
            .unwrap_or_else(|e| format!("Template error: {}", e)),
    )
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

fn origin(announcement: &TrainAnnouncement) -> String {
    join_locations(&announcement.from_location)
}

fn via(announcement: &TrainAnnouncement) -> String {
    join_locations(&announcement.via_to_location)
}

fn destination(announcement: &TrainAnnouncement) -> String {
    join_locations(&announcement.to_location)
}

fn join_locations(locations: &Vec<TrainLocation>) -> String {
    locations
        .iter()
        .map(|loc| name(&loc.location_name))
        .collect::<Vec<String>>()
        .join(", ")
}

fn product_information(announcement: &TrainAnnouncement) -> String {
    announcement
        .product_information
        .first()
        .map_or("".to_string(), |product| product.description.clone())
}

fn location(announcements: &Vec<TrainAnnouncement>) -> String {
    name(
        announcements
            .first()
            .map(|a| a.location_signature.as_str())
            .unwrap_or("Unknown"),
    )
}

fn train_ident(announcements: &Vec<TrainAnnouncement>) -> String {
    announcements
        .first()
        .map(|a| a.advertised_train_ident.clone())
        .unwrap_or("Unknown".to_string())
}

fn from(announcements: &Vec<TrainAnnouncement>) -> String {
    announcements
        .first()
        .map(origin)
        .unwrap_or("Unknown".to_string())
}

fn via_stations(announcements: &Vec<TrainAnnouncement>) -> String {
    announcements
        .first()
        .map(via)
        .unwrap_or("Unknown".to_string())
}

fn dest(announcements: &Vec<TrainAnnouncement>) -> String {
    announcements
        .first()
        .map(destination)
        .unwrap_or("Unknown".to_string())
}

fn prod(announcements: &Vec<TrainAnnouncement>) -> String {
    announcements
        .first()
        .map(product_information)
        .unwrap_or("Unknown".to_string())
}
