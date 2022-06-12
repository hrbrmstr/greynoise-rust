//! Make a request to the GreyNoise Tag Metadata API
use crate::get;

/// Structure to deserialize GreyNoise Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct TagMetadata {
    #[serde(rename = "metadata")]
    pub metadata: Vec<Metadatum>,

    #[serde(rename = "vpn_services")]
    pub vpn_services: Vec<String>,
}

/// Structure to deserialize individual GreyNoise Tag Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadatum {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "slug")]
    pub slug: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "category")]
    pub category: Category,

    #[serde(rename = "intention")]
    pub intention: Intention,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "references")]
    pub references: Vec<String>,

    #[serde(rename = "recommend_block")]
    pub recommend_block: bool,

    #[serde(rename = "cves")]
    pub cves: Vec<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "related_tags")]
    pub related_tags: Option<Vec<RelatedTag>>,
}

/// Structure to deserialize GreyNoise Tag Related Tags Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedTag {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "intention")]
    pub intention: Intention,

    #[serde(rename = "category")]
    pub category: Category,

    #[serde(rename = "slug")]
    pub slug: String,
}

/// Structure to deserialize GreyNoise Tag Category Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    #[serde(rename = "activity")]
    Activity,

    #[serde(rename = "actor")]
    Actor,

    #[serde(rename = "search_engine")]
    SearchEngine,

    #[serde(rename = "tool")]
    Tool,

    #[serde(rename = "worm")]
    Worm,
}

/// Structure to deserialize GreyNoise Tag Intention Metadata JSON.
///
/// Any values inside `Option` are optional, and you should check if there is `Some` or `None` before using it.
#[derive(Serialize, Deserialize, Debug)]
pub enum Intention {
    #[serde(rename = "benign")]
    Benign,

    #[serde(rename = "malicious")]
    Malicious,

    #[serde(rename = "unknown")]
    Unknown,
}

#[doc(hidden)]
const METADATA_URL: &str = "https://api.greynoise.io/v2/meta/metadata";

/// Function to retrieve list of GreyNoise tags and their respective metadata
///
/// There are limits to the daily use of the free RIOT API. Pass your GreyNoise API key to `key`
/// to expand the daily limit.
/// 
/// For more information on the Community API endpoint check the [API docs](https://docs.greynoise.io/reference/metadata-3).
///
/// # Example
/// ```rust
/// use greynoise::metadata;
/// async {
///  let res: Result<metadata::TagMetadata, reqwest::StatusCode> = metadata::metadata(None).await;
///  // assert_eq!(res.unwrap().ip, "8.8.8.8");
/// };
///```
///
/// # Errors
/// If the call fails, it will return a `Err(StatusCode)`.
/// To see the possible return values, check the [API docs](https://docs.greynoise.io/reference/metadata-3).
pub async fn metadata(key: Option<&str>) -> Result<TagMetadata, reqwest::StatusCode> {

  let url = format!("{}", METADATA_URL);
  let res: Result<TagMetadata, reqwest::StatusCode> = get::query(url, key).await;

  res

}

