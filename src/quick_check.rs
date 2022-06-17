//! Make a request to the GreyNoise IP Quick Check API
use crate::get;

pub type MultiQuickCheck = Vec<QuickCheck>;

/// Structure to deserialize GreyNoise Quick Check JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct QuickCheck {
    #[serde(rename = "ip")]
    pub ip: String,

    #[serde(rename = "noise")]
    pub noise: bool,

    #[serde(rename = "riot")]
    pub riot: bool,

    #[serde(rename = "code")]
    pub code: String,
}

#[doc(hidden)]
const QUICK_CHECK_CONTEXT_URL: &str = "https://api.greynoise.io/v2/noise/quick";

#[doc(hidden)]
const MULTI_QUICK_CHECK_CONTEXT_URL: &str = "https://api.greynoise.io/v2/noise/multi/quick";

/// Function to retrieve list of GreyNoise tags and their respective metadata
///
/// Check whether a given IP address is “Internet background noise”, or has been observed 
/// scanning or attacking devices across the Internet.
/// 
/// For more information on the IP Context API endpoint check the [API docs](https://docs.greynoise.io/reference/quickcheck-1).
///
/// # Example
/// ```rust
/// use greynoise::quick_check;
/// //async {
/// // let res: Result<quick_check::QuickCheck, reqwest::StatusCode> = quick_check::quick_check("71.6.233.151", Some(API_KEY)).await;
/// // assert_eq!(res.unwrap().ip, "71.6.233.151");
/// //};
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.greynoise.io/reference/quickcheck-1).
pub async fn quick_check(ip: &str, key: Option<&str>) -> Result<QuickCheck, reqwest::StatusCode> {

  let url = format!("{}/{}", QUICK_CHECK_CONTEXT_URL, ip);
  let res: Result<QuickCheck, reqwest::StatusCode> = get::query(url, key).await;

  res

}

pub async fn multi_quick_check(ips: Vec<String>, key: Option<&str>) -> Result<MultiQuickCheck, reqwest::StatusCode> {

  let url = format!("{}", MULTI_QUICK_CHECK_CONTEXT_URL);
  let res: Result<MultiQuickCheck, reqwest::StatusCode> = get::post_query(url, ips, key).await;

  res

}
