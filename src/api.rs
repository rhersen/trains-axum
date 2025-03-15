use crate::models::ApiResponse;
use std::error::Error;

fn xml_data(api_key: &str) -> String {
    let iso = "%Y-%m-%dT%H:%M:%S%.3f%:z";
    let now = chrono::Utc::now();
    let hour = chrono::Duration::hours(1);
    let since = (now - hour).format(iso);
    let until = (now + hour).format(iso);
    format!(
        r#"
<REQUEST>
  <LOGIN authenticationkey='{}' />
  <QUERY objecttype='TrainAnnouncement' orderby='AdvertisedTimeAtLocation' sseurl='false' schemaversion='1.6'>
    <FILTER>
      <AND>
        <EQ name='LocationSignature' value='Sk' />
        <GT name='AdvertisedTimeAtLocation' value='{}' />
        <LT name='AdvertisedTimeAtLocation' value='{}' />
      </AND>
    </FILTER>
    <INCLUDE>AdvertisedTrainIdent</INCLUDE>
    <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
    <INCLUDE>TimeAtLocationWithSeconds</INCLUDE>
    <INCLUDE>ToLocation</INCLUDE>
  </QUERY>
</REQUEST>
"#,
        api_key, since, until
    )
}

fn build_request(xml_data: String) -> reqwest::RequestBuilder {
    reqwest::Client::new()
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
