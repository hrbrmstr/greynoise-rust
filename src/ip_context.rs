//! Make a request to the GreyNoise Tag Metadata API
use crate::get;

/// Structure to deserialize GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct IPContext {
    #[serde(rename = "ip")]
    pub ip: String,

    #[serde(rename = "first_seen")]
    pub first_seen: Option<String>,

    #[serde(rename = "last_seen")]
    pub last_seen: Option<String>,

    #[serde(rename = "seen")]
    pub seen: bool,

    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "actor")]
    pub actor: Option<String>,

    #[serde(rename = "spoofable")]
    pub spoofable: Option<bool>,

    #[serde(rename = "classification")]
    pub classification: Option<String>,

    #[serde(rename = "cve")]
    pub cve: Option<Vec<String>>,

    #[serde(rename = "bot")]
    pub bot: Option<bool>,

    #[serde(rename = "vpn")]
    pub vpn: Option<bool>,

    #[serde(rename = "vpn_service")]
    pub vpn_service: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "raw_data")]
    pub raw_data: Option<RawData>,
}

/// Structure to deserialize base metadata of GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
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

/// Structure to deserialize raw metadata of GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
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

/// Structure to deserialize scan metadata of GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct Scan {
    #[serde(rename = "port")]
    pub port: Option<i64>,

    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
}

/// Structure to deserialize Web metadata of GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct Web {
  #[serde(rename = "scan")]
  pub paths: Option<Vec<String>>,

  #[serde(rename = "useragents")]
  pub usergents: Option<Vec<String>>
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
const IP_CONTEXT_URL: &str = "https://api.greynoise.io/v2/noise/context";

/// Function to retrieve list of GreyNoise tags and their respective metadata
///
/// Get more information about a given IP address. Returns time ranges, IP metadata 
/// (network owner, ASN, reverse DNS pointer, country), associated actors, activity 
/// tags, and raw port scan and web request information.
/// 
/// For more information on the IP Context API endpoint check the [API docs](https://docs.greynoise.io/reference/noisecontextip-1).
///
/// # Example
/// ```rust
/// use greynoise::ip_context;
/// //async {
///  //let res: Result<ip_context::IPContext, reqwest::StatusCode> = ip_context::ip_context("71.6.233.151", Some(API_KEY)).await;
///  // assert_eq!(res.unwrap().ip, "8.8.8.8");
/// //};
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.greynoise.io/reference/noisecontextip-1).
pub async fn ip_context(ip: &str, key: Option<&str>) -> Result<IPContext, reqwest::StatusCode> {

  let url = format!("{}/{}", IP_CONTEXT_URL, ip);
  let res: Result<IPContext, reqwest::StatusCode> = get::query(url, key).await;

  res

}
