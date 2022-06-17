//! Make a request to the GreyNoise Tag Metadata API
use crate::get;

#[derive(Debug, Serialize, Deserialize)]
pub struct QNQL {
  #[serde(rename = "complete")]
  pub complete: Option<bool>,
  
  #[serde(rename = "count")]
  pub count: i64,
  
  #[serde(rename = "data")]
  pub data: Vec<Datum>,
  
  #[serde(rename = "message")]
  pub message: String,
  
  #[serde(rename = "query")]
  pub query: String,

  #[serde(rename = "scroll")]
  pub scroll: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
  #[serde(rename = "ip")]
  pub ip: String,
  
  #[serde(rename = "metadata")]
  pub metadata: Option<Metadata>,
  
  #[serde(rename = "bot")]
  pub bot: Option<bool>,
  
  #[serde(rename = "vpn")]
  pub vpn: Option<bool>,
  
  #[serde(rename = "vpn_service")]
  pub vpn_service: Option<String>,
  
  #[serde(rename = "spoofable")]
  pub spoofable: Option<bool>,
  
  #[serde(rename = "raw_data")]
  pub raw_data: Option<RawData>,
  
  #[serde(rename = "first_seen")]
  pub first_seen: Option<String>,
  
  #[serde(rename = "last_seen")]
  pub last_seen: Option<String>,
  
  #[serde(rename = "seen")]
  pub seen: Option<bool>,
  
  #[serde(rename = "tags")]
  pub tags: Option<Vec<String>>,
  
  #[serde(rename = "actor")]
  pub actor: Option<String>,
  
  #[serde(rename = "classification")]
  pub classification: Option<String>,
  
  #[serde(rename = "cve")]
  pub cve: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  #[serde(rename = "asn")]
  pub asn: Option<String>,
  
  #[serde(rename = "city")]
  pub city: Option<String>,
  
  #[serde(rename = "country")]
  pub country: Option<String>,
  
  #[serde(rename = "country_code")]
  pub country_code: Option<String>,
  
  #[serde(rename = "organization")]
  pub organization: Option<String>,
  
  #[serde(rename = "category")]
  pub category: Option<String>,
  
  #[serde(rename = "tor")]
  pub tor: Option<bool>,
  
  #[serde(rename = "rdns")]
  pub rdns: Option<String>,
  
  #[serde(rename = "os")]
  pub os: Option<String>,
  
  #[serde(rename = "region")]
  pub region: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawData {
  #[serde(rename = "scan")]
  pub scan: Option<Vec<Scan>>,
  
  #[serde(rename = "web")]
  pub web: Option<Web>,
  
  #[serde(rename = "ja3")]
  pub ja3: Option<Vec<Option<JA3>>>,
  
  #[serde(rename = "hassh")]
  pub hassh: Option<Vec<Option<HASSH>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
  #[serde(rename = "port")]
  pub port: Option<i64>,
  
  #[serde(rename = "protocol")]
  pub protocol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Web {
  #[serde(rename = "paths")]
  pub paths: Option<String>,

  #[serde(rename = "useragents")]
  pub useragents: Option<String>,
}

/// Structure to deserialize ja3 hash metadata of GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct JA3 {
  #[serde(rename = "fingerprint")]
  pub fingerprint: Option<String>,

  #[serde(rename = "port")]
  pub port: Option<i64>
}

/// Structure to deserialize hassh base metadata of GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct HASSH {
  #[serde(rename = "fingerprint")]
  pub fingerprint: Option<String>,

  #[serde(rename = "port")]
  pub port: Option<i64>
}

#[doc(hidden)]
const QNQL_QUERY_URL: &str = "https://api.greynoise.io/v2/experimental/gnql";

/// Function to retrieve list of GreyNoise tags and their respective metadata
///
/// Get more information about a given IP address. Returns time ranges, IP metadata 
/// (network owner, ASN, reverse DNS pointer, country), associated actors, activity 
/// tags, and raw port scan and web request information.
/// 
/// For more information on the IP Context API endpoint check the [API docs](https://api.greynoise.io/v2/experimental/gnql).
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](ttps://api.greynoise.io/v2/experimental/gnql).
pub async fn gnql_query(query: &str, size: Option<i64>, scroll: Option<&str>, key: Option<&str>) -> Result<QNQL, reqwest::StatusCode> {

  let size_param: String = if let Some(size) = size { format!("&size={}", size.to_string()) } else { format!("") };
  let scroll: String = if let Some(scroll) = scroll { format!("&scroll={}", scroll.to_string()) } else { format!("") } ;
  let url = format!("{}?query={}{}{}", QNQL_QUERY_URL, query, size_param, scroll);

  let res: Result<QNQL, reqwest::StatusCode> = get::query(url, key).await;

  res

}
