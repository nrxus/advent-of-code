use cookie::Cookie;
use reqwest::{header, Client};
use std::env;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    EnvVariableIncorrect(env::VarError),
}

pub fn fetch_or_exit(day: u8) -> String {
    match try_fetch(day) {
        Ok(i) => i,
        Err(Error::Request(e)) => {
            eprintln!("error: request to get input failed with error: {}", e);
            std::process::exit(1);
        }
        Err(Error::EnvVariableIncorrect(e)) => {
            eprintln!("error: could not retrieve session cookie from 'AOC_SESSION_COOKIE' env variable. Failed with error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn try_fetch(day: u8) -> Result<String, Error> {
    let session_string = env::var("AOC_SESSION_COOKIE").map_err(Error::EnvVariableIncorrect)?;
    let client = Client::new();
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let cookie = Cookie::new("session", session_string);
    client
        .get(&url)
        .header(header::COOKIE, cookie.to_string())
        .send()
        .and_then(|mut r| r.text())
        .map_err(Error::Request)
}
