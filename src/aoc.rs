use std::{io::{BufRead, BufReader}, path::Path};

use reqwest::StatusCode;

pub fn get_input(year: i32, day: i32) -> Result<Vec<String>, AocError> {
    create_cache_dir()?;
    let file_path = format!(".cache/{year}-{day}.txt");
    if Path::exists(Path::new(".cache/{}-{}.txt")) {
        let mut result = Vec::<String>::new();
        for line in std::fs::read_to_string(file_path)?.lines() {
            result.push(line.to_string());
        }
        return Ok(result);
    }
    return get_input_from_http_and_cache(year, day);
}

pub fn get_input_from_http_and_cache(year: i32, day: i32) -> Result<Vec<String>, AocError> {
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
    // TODO: Save cache stuff
    Ok(res)
}

fn read_session_cookie() -> Result<String, AocError> {
    Ok(std::fs::read_to_string("session.txt")?)
}

fn create_cache_dir() -> std::io::Result<()> {
    if Path::new(".cache").is_dir() {
        return Ok(());
    }
    Ok(std::fs::create_dir(".cache")?)
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