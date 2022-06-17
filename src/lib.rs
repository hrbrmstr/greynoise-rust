//! The `greynoise` crate provides a Rust wrapper (SDK) for the [GreyNoise API](https://docs.greynoise.io/reference/).

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ini;

pub mod gn;
pub mod community;
pub mod ip_context;
pub mod quick_check;
pub mod riot;
pub mod metadata;
pub mod ping;
pub mod gnql;
mod get;

#[allow(unused_macros)]
macro_rules! vec_of_strings {
  ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod tests {

  use crate::*;
  
  #[tokio::test]
  async fn community_works() {
    let res: Result<community::Community, reqwest::StatusCode> = community::community("8.8.8.8", None).await;
    assert_eq!(res.unwrap().ip, "8.8.8.8");
  }

  #[tokio::test]
  async fn community_with_key_works() {
    let res: Result<community::Community, reqwest::StatusCode> = community::community("8.8.8.8", Some(gn::api_key(None).as_ref())).await;
    assert_eq!(res.unwrap().ip, "8.8.8.8");
  }

  #[tokio::test]
  async fn community_fails() {
    let res: Result<community::Community, reqwest::StatusCode> = community::community("eights", None).await;
    match res {
      Ok(_) => { assert!(false)}
      Err(_) => { assert!(true)}
    }
  }

  #[tokio::test]
  async fn community_with_key_fails() {
    let res: Result<community::Community, reqwest::StatusCode> = community::community("eights", Some(gn::api_key(None).as_ref())).await;
    match res {
      Ok(_) => { assert!(false)}
      Err(_) => { assert!(true)}
    }
  }

  // #[tokio::test]
  // async fn riot_works() {
  //   let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("8.8.8.8", None).await;
  //   assert_eq!(res.unwrap().ip, "8.8.8.8");
  // }

  #[tokio::test]
  async fn riot_with_key_works() {
    let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("8.8.8.8", Some(gn::api_key(None).as_ref())).await;
    assert_eq!(res.unwrap().ip, "8.8.8.8");
  }

  #[tokio::test]
  async fn riot_fails() {
    let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("eights", None).await;
    match res {
      Ok(_) => { assert!(false)}
      Err(_) => { assert!(true)}
    }
  }

  #[tokio::test]
  async fn riot_with_key_fails() {
    let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("eights", Some(gn::api_key(None).as_ref())).await;
    match res {
      Ok(_) => { assert!(false)}
      Err(_) => { assert!(true)}
    }
  }

  #[tokio::test]
  async fn metadata_with_key_works() {
    let res: Result<metadata::TagMetadata, reqwest::StatusCode> = metadata::metadata(Some(gn::api_key(None).as_ref())).await;
    assert!(res.unwrap().metadata.len() > 0);
  }

  #[tokio::test]
  async fn ip_context_works() {
    
    let res: Result<ip_context::IPContext, reqwest::StatusCode> = ip_context::ip_context("71.6.233.151", Some(gn::api_key(None).as_ref())).await;

    match res {
      Ok(_) => { assert!(true)}
      Err(err) => { 
        eprintln!("ERROR: {}", err.as_str());
        assert!(false)
      }
    }

  }

  #[tokio::test]
  async fn quick_check_with_key_works() {
    let res: Result<quick_check::QuickCheck, reqwest::StatusCode> = quick_check::quick_check("71.6.233.151", Some(gn::api_key(None).as_ref())).await;
    assert_eq!(res.unwrap().ip, "71.6.233.151");
  }


  #[tokio::test]
  async fn multi_quick_check_with_key_works() {
    let ips: Vec<String> = vec_of_strings![ "8.8.8.8", "8.8.4.4" ];
    let res: Result<quick_check::MultiQuickCheck, reqwest::StatusCode> = quick_check::multi_quick_check(ips, Some(gn::api_key(None).as_ref())).await;
    match res {
      Ok(_) => { assert_eq!(res.unwrap().len(), 2) }
      Err(err) => { 
        eprintln!("ERROR: {}", err.as_str());
        assert!(false)
      }
    }
  }

  #[tokio::test]
  async fn check_ping() {
    assert!(ping::ping(Some(gn::api_key(None).as_ref())).await)
  }


  #[tokio::test]
  async fn gnql_query_with_key_works() {
    let res: Result<gnql::QNQL, reqwest::StatusCode> = gnql::gnql_query("tags:Mirai", Some(100), Some("scrolly"), Some(gn::api_key(None).as_ref())).await;
    assert_eq!(res.unwrap().query, "tags:Mirai");
  }


}
