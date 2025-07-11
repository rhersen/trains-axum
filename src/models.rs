use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "RESPONSE")]
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "RESULT")]
    pub result: Vec<Result>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "TrainAnnouncement")]
    pub train_announcements: Vec<TrainAnnouncement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainAnnouncement {
    #[serde(rename = "AdvertisedTrainIdent")]
    pub advertised_train_ident: String,

    #[serde(rename = "AdvertisedTimeAtLocation")]
    pub advertised_time_at_location: DateTime<FixedOffset>,

    #[serde(rename = "LocationSignature")]
    pub location_signature: String,

    #[serde(rename = "TimeAtLocationWithSeconds")]
    pub time_at_location_with_seconds: Option<DateTime<FixedOffset>>,

    #[serde(rename = "ToLocation", default)]
    pub to_location: Vec<TrainLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainLocation {
    #[serde(rename = "LocationName")]
    pub location_name: String,

    #[serde(rename = "Priority")]
    pub priority: i32,

    #[serde(rename = "Order")]
    pub order: i32,
}
