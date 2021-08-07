use juniper::{graphql_object, FieldResult};
use serde::{Serialize, Deserialize};

use crate::database::Context;
use crate::models::{region::Region};

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

    async fn code_regions() -> FieldResult<Option<Vec<Region>>> {

        let data = reqwest::get("https://geo.api.gouv.fr/regions")
            .await?
            .text()
            .await?;

        let regions: Vec<Region> = serde_json::from_str(data.as_str()).expect("json string to region struct");

        Ok(Some(regions))

    }
}