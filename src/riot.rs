//! Make a request to the GreyNoise RIOT API
use crate::get;

/// Structure to deserialize GreyNoise RIOT JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct RIOT {
    #[serde(rename = "ip")]
    pub ip: String,

    #[serde(rename = "riot")]
    pub riot: bool,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "explanation")]
    pub explanation: String,

    #[serde(rename = "last_updated")]
    pub last_updated: String,

    #[serde(rename = "reference")]
    pub reference: String,

    #[serde(rename = "trust_level")]
    pub trust_level: String,
}

#[doc(hidden)]
const RIOT_URL: &str = "https://api.greynoise.io/v2/riot";

/// Function to retrieve information about an IP address in the GreyNoise RIOT dataset
///
/// RIOT identifies IPs from known benign services and organizations that commonly cause 
/// false positives in network security and threat intelligence products. The collection 
/// of IPs in RIOT is continually curated and verified to provide accurate results.
///
/// There are limits to the daily use of the free RIOT API. Pass your GreyNoise API key to `key`
/// to expand the daily limit.
/// 
/// For more information on the Community API endpoint check the [API docs](https://docs.greynoise.io/reference/riotip).
///
/// # Example
/// ```rust
/// use greynoise::riot;
/// async {
///  let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("8.8.8.8", None).await;
///  assert_eq!(res.unwrap().ip, "8.8.8.8");
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.greynoise.io/reference/riotip).
pub async fn riot(ip: &str, key: Option<&str>) -> Result<RIOT, reqwest::StatusCode> {

  let url = format!("{}/{}", RIOT_URL, ip);
  let res: Result<RIOT, reqwest::StatusCode> = get::query(url, key).await;

  res

}

