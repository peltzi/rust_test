use reqwest::blocking::{Response};
use std::error::Error;

pub fn http_get(uri: &str) -> Result<Response, Box<dyn Error>> {
    let resp = reqwest::blocking::get(uri)?;
    Ok(resp)
}
