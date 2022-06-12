//! Make a request to the GreyNoise Community API
use crate::get;

/// Structure to deserialize GreyNoise Community JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct Community {
  #[serde(rename = "ip")]
  pub ip: String,
  
  #[serde(rename = "noise")]
  pub noise: bool,
  
  #[serde(rename = "riot")]
  pub riot: bool,
  
  #[serde(rename = "classification")]
  pub classification: String,
  
  #[serde(rename = "name")]
  pub name: String,
  
  #[serde(rename = "link")]
  pub link: String,
  
  #[serde(rename = "last_seen")]
  pub last_seen: String,
  
  #[serde(rename = "message")]
  pub message: String,
}

#[doc(hidden)]
const COMMUNITY_URL: &str = "https://api.greynoise.io/v3/community";

/// Function to retrieve information about an IP address in the GreyNoise dataset
///
/// The Community API provides community users with a free tool to query IPs in the
/// GreyNoise dataset and retrieve a subset of the full IP context data returned by the IP Lookup API.
///
/// There are limits to the daily use of the free Community API. Pass your GreyNoise API key to `key`
/// to expand the daily limit.
/// 
/// For more information on the Community API endpoint check the [API docs](https://docs.greynoise.io/reference/get_v3-community-ip?).
///
/// # Example
/// ```rust
/// use greynoise::community;
/// async {
///  let res: Result<community::Community, reqwest::StatusCode> = community::community("8.8.8.8", None).await;
///  assert_eq!(res.unwrap().ip, "8.8.8.8");
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.greynoise.io/reference/get_v3-community-ip?).
pub async fn community(ip: &str, key: Option<&str>) -> Result<Community, reqwest::StatusCode> {

  let url = format!("{}/{}", COMMUNITY_URL, ip);
  let res: Result<Community, reqwest::StatusCode> = get::query(url, key).await;

  res

}

