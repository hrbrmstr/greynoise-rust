//! The `greynoise` crate provides a Rust wrapper (SDK) for the [GreyNoise API](https://docs.greynoise.io/reference/).

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ini;

pub mod greynoise;
pub mod community;
pub mod ip_context;
pub mod quick_check;
pub mod riot;
pub mod metadata;
mod get;


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
    let res: Result<community::Community, reqwest::StatusCode> = community::community("8.8.8.8", Some(greynoise::api_key(None).as_ref())).await;
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
    let res: Result<community::Community, reqwest::StatusCode> = community::community("eights", Some(greynoise::api_key(None).as_ref())).await;
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
    let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("8.8.8.8", Some(greynoise::api_key(None).as_ref())).await;
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
    let res: Result<riot::RIOT, reqwest::StatusCode> = riot::riot("eights", Some(greynoise::api_key(None).as_ref())).await;
    match res {
      Ok(_) => { assert!(false)}
      Err(_) => { assert!(true)}
    }
  }

  #[tokio::test]
  async fn metadata_with_key_works() {
    let res: Result<metadata::TagMetadata, reqwest::StatusCode> = metadata::metadata(Some(greynoise::api_key(None).as_ref())).await;
    assert!(res.unwrap().metadata.len() > 0);
  }

  #[tokio::test]
  async fn ip_context_works() {
    
    let res: Result<ip_context::IPContext, reqwest::StatusCode> = ip_context::ip_context("71.6.233.151", Some(greynoise::api_key(None).as_ref())).await;

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
    let res: Result<quick_check::QuickCheck, reqwest::StatusCode> = quick_check::quick_check("71.6.233.151", Some(greynoise::api_key(None).as_ref())).await;
    assert_eq!(res.unwrap().ip, "71.6.233.151");
  }

}
