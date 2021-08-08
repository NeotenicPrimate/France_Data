use juniper::{graphql_object, FieldResult};
use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap};
use html_parser::{Dom};
use serde_json::{Value};

use crate::database::Context;
use crate::models::{region::Region};
use crate::data_types::{time_series::{TimeSeriesDataPoint}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
  pub name: String,
  pub alpha2Code: String,
  pub alpha3Code: String,
  pub population: Option<i32>,
  pub gini: Option<f64>,
}

#[graphql_object(context = Context)]
impl Country {
    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn alpha2Code(&self) -> &str {
        self.alpha2Code.as_str()
    }

    async fn alpha3Code(&self) -> &str {
        self.alpha3Code.as_str()
    }

    async fn population(&self) -> Option<i32> {
        match self.population {
            Some(value) => Some(value),
            None => None,
        }
    }

    async fn gini(&self) -> Option<f64> {
        match self.gini {
            Some(value) => Some(value),
            None => None,
        }
    }

    async fn chommage(&self) -> FieldResult<Option<Vec<TimeSeriesDataPoint>>> {
          
        let client = reqwest::Client::new();
        
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/xml".parse().unwrap());
        headers.insert("authorization", "Bearer af7bbf6c-4392-3726-90f1-6959447492ba".parse().unwrap());
    
        let data = client
            .get("https://api.insee.fr/series/BDM/V1/data/SERIES_BDM/010605048")
            .headers(headers)
            .send()
            .await.expect("Error 1")
            .text()
            .await.expect("Error 2");
    
        let json_string = Dom::parse(data.as_str()).expect("parse").to_json().expect("to json");
        let root: Value = serde_json::from_str(&json_string).expect("deserialize");
    
        let children0 = root.get("children").expect("no records");
        let children1 = children0[0].get("children").expect("key does not exist");
        let children2 = children1[1].get("children").expect("key does not exist");
        let children3 = children2[0].get("children").expect("key does not exist");
    
        let mut time_series = Vec::new();
        for obs in children3.as_array().expect("array") {
            let data_point: TimeSeriesDataPoint = serde_json::from_value(obs["attributes"].to_owned()).expect("json string to Datapoint");
            time_series.push(data_point);
        };

        Ok(Some(time_series))

    }

    async fn code_regions() -> FieldResult<Option<Vec<Region>>> {

        let data = reqwest::get("https://geo.api.gouv.fr/regions")
            .await?
            .text()
            .await?;

        let regions: Vec<Region> = serde_json::from_str(data.as_str()).expect("json string to region struct");

        Ok(Some(regions))

    }
}