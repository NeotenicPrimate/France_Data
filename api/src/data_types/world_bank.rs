use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject};
use reqwest::header::{HeaderMap};
use scraper::{Html, Selector};
use serde_json::{Value};

/// To find variables: https://data.worldbank.org/
pub async fn request_world_bank(
    country_code: &String,
    variables: Vec<&str>,
    date: Vec<i32>,
) -> Option<f64> {
    let client = reqwest::Client::new();
  
    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/xml".parse().unwrap());

    let client = reqwest::Client::new();
  
    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/xml".parse().unwrap());

    let date: Vec<String> = date.iter().map(|i| i.to_string()).collect();
    let date = date.join(":");

    let json = client
        .get(format!("https://api.worldbank.org/v2/country/{}/indicator/{}?date={}&format=json", 
            country_code, 
            variables.join("."),
            date, 
        ).as_str())
        .headers(headers)
        .send()
        .await.expect("Error 1")
        .text()
        .await.expect("Error 2");

    let json: Value = serde_json::from_str(json.as_str()).expect("convert serde_json value");

    let value = json[1][0]["value"].as_f64();

    match value {
        Some(v) => return Some(v),
        None => return None,
    };
  
  }