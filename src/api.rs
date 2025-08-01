use crate::models::ApiResponse;
use std::error::Error;

const ISO: &'static str = "%Y-%m-%dT%H:%M:%S%:z";
const SINCE: chrono::TimeDelta = chrono::Duration::hours(4);
const UNTIL: chrono::TimeDelta = chrono::Duration::hours(12);

pub async fn fetch_station(api_key: &str, code: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let data = station_query(api_key, code);
    let response = build_request(data).send().await?;
    let text = response.text().await?;
    let parsed: ApiResponse = serde_json::from_str(&text)?;
    Ok(parsed)
}

fn station_query(api_key: &str, code: &str) -> String {
    let now = chrono::Utc::now();
    let since = (now - SINCE).format(ISO);
    let until = (now + UNTIL).format(ISO);
    format!(
        r#"
<REQUEST>
  <LOGIN authenticationkey='{}' />
  <QUERY objecttype='TrainAnnouncement' orderby='AdvertisedTimeAtLocation' sseurl='false' schemaversion='1.6'>
    <FILTER>
      <AND>
        <NE name='Canceled' value='true' />
        <EQ name='Advertised' value='true' />
        <EQ name='ActivityType' value='Avgang' />
        <EQ name='LocationSignature' value='{}' />
        <GT name='AdvertisedTimeAtLocation' value='{}' />
        <LT name='AdvertisedTimeAtLocation' value='{}' />
      </AND>
    </FILTER>
    <INCLUDE>AdvertisedTrainIdent</INCLUDE>
    <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
    <INCLUDE>FromLocation</INCLUDE>
    <INCLUDE>LocationSignature</INCLUDE>
    <INCLUDE>ProductInformation</INCLUDE>
    <INCLUDE>TimeAtLocationWithSeconds</INCLUDE>
    <INCLUDE>ToLocation</INCLUDE>
    <INCLUDE>ViaToLocation</INCLUDE>
  </QUERY>
</REQUEST>
"#,
        api_key, code, since, until
    )
}

pub async fn fetch_train(api_key: &str, code: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let data = train_query(api_key, code);
    let response = build_request(data).send().await?;
    let text = response.text().await?;
    let parsed: ApiResponse = serde_json::from_str(&text)?;
    Ok(parsed)
}

fn train_query(api_key: &str, id: &str) -> String {
    let now = chrono::Utc::now();
    let since = (now - SINCE).format(ISO);
    let until = (now + UNTIL).format(ISO);
    format!(
        r#"
<REQUEST>
  <LOGIN authenticationkey='{}' />
  <QUERY objecttype='TrainAnnouncement' orderby='AdvertisedTimeAtLocation' sseurl='false' schemaversion='1.6'>
    <FILTER>
      <AND>
        <NE name='Canceled' value='true' />
        <EQ name='Advertised' value='true' />
        <EQ name='ActivityType' value='Avgang' />
        <EQ name='AdvertisedTrainIdent' value='{}' />
        <GT name='AdvertisedTimeAtLocation' value='{}' />
        <LT name='AdvertisedTimeAtLocation' value='{}' />
      </AND>
    </FILTER>
    <INCLUDE>AdvertisedTrainIdent</INCLUDE>
    <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
    <INCLUDE>FromLocation</INCLUDE>
    <INCLUDE>LocationSignature</INCLUDE>
    <INCLUDE>ProductInformation</INCLUDE>
    <INCLUDE>TimeAtLocationWithSeconds</INCLUDE>
    <INCLUDE>ToLocation</INCLUDE>
    <INCLUDE>ViaToLocation</INCLUDE>
  </QUERY>
</REQUEST>
"#,
        api_key, id, since, until
    )
}

fn build_request(xml_data: String) -> reqwest::RequestBuilder {
    reqwest::Client::new()
        .post("https://api.trafikinfo.trafikverket.se/v2/data.json")
        .header("Content-Type", "application/xml")
        .body(xml_data)
}
