use std::collections::HashMap;

/// Retrieve the GreyNoise API client
///  configuration from the local config file
pub fn get_config(path: Option<String>) -> HashMap<String, HashMap<String, Option<String>>> {

  let map: HashMap<String, HashMap<String, Option<String>>>;
  
  if let Some(p) = path {
    let p_str: &str = p.as_ref();
    map = ini::ini!(shellexpand::tilde(p_str).as_ref());
  } else {
    map = ini::ini!(shellexpand::tilde("~/.config/greynoise/config").as_ref());
  }

  return map;

}

/// Retrieve the GreyNoise API key from the local config file
pub fn api_key(path: Option<String>) -> String {

  let map: HashMap<String, HashMap<String, Option<String>>>;
  
  if let Some(p) = path {
    let p_str: &str = p.as_ref();
    map = ini::ini!(shellexpand::tilde(p_str).as_ref());
  } else {
    map = ini::ini!(shellexpand::tilde("~/.config/greynoise/config").as_ref());
  }

  return map["greynoise"]["api_key"].clone().unwrap();

}
