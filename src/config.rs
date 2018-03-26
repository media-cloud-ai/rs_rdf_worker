
use std::env;

macro_rules! get_env_value {
  ($key:expr, $default:expr) => (
  {
    let mut item = $default.to_string();
    for (key, value) in env::vars() {
      match key.as_ref() {
        $key => {
          item = value;
        }
        _ => {},
      }
    }
    item
  })
}

pub fn get_hostname() -> String {
  get_env_value!("HOSTNAME", "127.0.0.1")
}

pub fn get_port() -> String {
  get_env_value!("PORT", "1501")
}
