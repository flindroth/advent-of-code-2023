use std::io::{BufRead, BufReader};

use reqwest::StatusCode;

pub fn get_input(year: i32, day: i32) -> Result<Vec<String>, AocError> {
    let session_cookie = read_session_cookie()?;
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;

    if resp.status() != StatusCode::OK {
        return Err(AocError::NotFound);
    }

    let reader = BufReader::new(resp);
    let mut res = Vec::<String>::new();
    for line in reader.lines() {
        res.push(line?);
    }
    Ok(res)
}

fn read_session_cookie() -> Result<String, AocError> {
    Ok(std::fs::read_to_string("session.txt")?)
}

#[derive(Debug)]
pub enum AocError {
    NotFound,
    IoError
}

impl From<reqwest::Error> for AocError {
    fn from(_value: reqwest::Error) -> Self {
        AocError::IoError
    }
}

impl From<std::io::Error> for AocError {
    fn from(_value: std::io::Error) -> Self {
        AocError::IoError
    }
}