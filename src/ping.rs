//! Check GreyNoise API Status
use crate::get;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping {
  #[serde(rename = "expiration")]
  expiration: Option<String>,
  
  #[serde(rename = "message")]
  message: Option<String>,
  
  #[serde(rename = "offering")]
  offering: Option<String>,
}

#[doc(hidden)]
const PING_URL: &str = "https://api.greynoise.io/ping";

/// Function to test if the GreyNoise API is alive
///
/// Provides a simple endpoint to check GreyNoise status and GreyNoise API access
/// 
/// For more information on the Ping API endpoint check the [API docs](https://docs.greynoise.io/reference/get_ping).
///
/// # Example
/// ```rust
/// use greynoise::ping;
/// 
/// async {
///  let res: bool = ping::ping(Some(greynoise::gn::api_key(None).as_ref())).await;
///   assert!(res);
/// };
///```
///
/// # Errors
/// If the call fails, it will return `false`.
/// To see the possible return values, check the [API docs](https://docs.greynoise.io/reference/get_ping).

pub async fn ping(key: Option<&str>) -> bool {
  
  let url = format!("{}", PING_URL);
  let res: Result<Ping, reqwest::StatusCode> = get::query(url, key).await;
  
  match res {
    Ok(_) => { return true }
    Err(_) => { return false }
  }
  
}
