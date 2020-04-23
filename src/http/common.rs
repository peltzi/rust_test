use reqwest::blocking::{Response};
use serde_json::{Value};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CheckError {
    message: String,
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Response check error: {}",
            self.message
            )
    }
}

pub fn check_error(resp: Response) -> Result<Response, CheckError> {
    if resp.status().is_success() {
        Ok(resp)
    } else if resp.status().is_server_error() {
        Err(CheckError{message: format!("{:?}", resp)})
    } else {
        Err(CheckError{message: format!("{:?}", resp)})
    }
}

pub fn parse_json(result: Response) -> Result<Value, Box<dyn Error>> {
    let json: Value = result.json()?;
    Ok(json)
}
