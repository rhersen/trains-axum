use crate::models::ApiResponse;
use reqwest::Client;
use std::error::Error;

pub fn xml_data(api_key: &str) -> String {
    format!(
        r#"
<REQUEST>
  <LOGIN authenticationkey='{}' />
  <QUERY objecttype='TrainAnnouncement' sseurl='false' schemaversion='1.6'>
    <FILTER>
      <AND>
        <EQ name='LocationSignature' value='Sk' />
        <GT name='AdvertisedTimeAtLocation' value='2025-03-27T04:00Z' />
      </AND>
    </FILTER>
    <INCLUDE>AdvertisedTrainIdent</INCLUDE>
    <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
    <INCLUDE>ToLocation</INCLUDE>
  </QUERY>
</REQUEST>
"#,
        api_key
    )
}

pub fn build_request(xml_data: String) -> reqwest::RequestBuilder {
    Client::new()
        .post("https://api.trafikinfo.trafikverket.se/v2/data.json")
        .header("Content-Type", "application/xml")
        .body(xml_data)
}

pub async fn fetch_announcements(api_key: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let data = xml_data(api_key);
    let response = build_request(data).send().await?;
    let text = response.text().await?;
    let parsed: ApiResponse = serde_json::from_str(&text)?;
    Ok(parsed)
}
