use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[doc(hidden)]
pub async fn query<T>(url: String, key: Option<&str>) -> Result<T, StatusCode> 
  where T: DeserializeOwned, {

  let client = reqwest::Client::new();

  let mut call = client.get(url);

  if let Some(k) = key {
    call = call.header("key", k)
  }
  
  let resp = call.send().await;

  match &resp {
    Ok(r) => {
      if r.status() != StatusCode::OK {
        return Err(r.status());
      }
    }
    Err(e) => {
      if e.is_status() {
        return Err(e.status().unwrap());
      } else {
        return Err(StatusCode::BAD_REQUEST);
      }
    }
  }
  
  // Parse the response body as Json
  let content = resp.unwrap().json::<T>().await;
  
  match content {
    Ok(s) => Ok(s),
    Err(e) => {
      println!("{:?}", e);
      Err(StatusCode::BAD_REQUEST)
    }
  }
  
}

#[derive(Serialize, Deserialize, Debug)]
struct IPVec {
  ips: Vec<String>
}

#[doc(hidden)]
pub async fn post_query<T>(url: String, ips: Vec<String>, key: Option<&str>) -> Result<T, StatusCode> 
  where T: DeserializeOwned, {
     
  let body = IPVec {
    ips: ips.clone()
  };

  // let client = reqwest::Client::new();
  let builder = reqwest::ClientBuilder::new();

  let client = builder.connection_verbose(true).build().expect("Could not build client");

  println!("{:?}", serde_json::to_vec(&body));

  let mut call = client.post(url).json(&body);

  if let Some(k) = key {
    call = call.header("key", k)
  }
  
  let resp = call.send().await;

  match &resp {
    Ok(r) => {
      if r.status() != StatusCode::OK {
        return Err(r.status());
      }
    }
    Err(e) => {
      if e.is_status() {
        return Err(e.status().unwrap());
      } else {
        return Err(StatusCode::BAD_REQUEST);
      }
    }
  }
  
  // Parse the response body as Json
  let content = resp.unwrap().json::<T>().await;
  
  match content {
    Ok(s) => Ok(s),
    Err(e) => {
      println!("{:?}", e);
      Err(StatusCode::BAD_REQUEST)
    }
  }
  
}