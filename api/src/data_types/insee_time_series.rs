use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject};
use reqwest::header::{HeaderMap};
use scraper::{Html, Selector};
use serde_json::{Value};

// RP Recensement de la population 1999, de 2006 à 2018
// BDCOM Séries historiques du recensement de la population (depuis 1968) de 2012 à 2018
// Popleg Populations légales (issue du recensement de la population) de 2006 à 2018
// RFD Données de l'état-civil (naissances et décès) de 2007 à 2019
// REE Répertoire des entreprises et des établissements (issu de Sirene) de 2009 à 2020
// FILOSOFI Fichier localisé social et scal 2017 et 2018
// Flores Fichier localisé des rémunérations et de l'emploi salarié 2017 et 2018
// TOUR Tourisme (ore d'hébergement) de 2013 à 2021


#[derive(Serialize, Deserialize, GraphQLObject, Debug)]
pub struct TimeSeriesDataPoint {
    obs_qual: Option<String>,
    obs_status: Option<String>,
    obs_type: Option<String>,
    obs_value: Option<String>,
    time_period: Option<String>,
}

/// BDM - V1
/// https://api.insee.fr/catalogue/site/themes/wso2/subthemes/insee/pages/item-info.jag?name=BDM&version=V1&provider=insee#!/Series_chronologiques/get_data_SERIES_BDM_idbanks
pub async fn request_insee_bdm(id: &str) -> Vec<TimeSeriesDataPoint> {
    let client = reqwest::Client::new();
  
    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/xml".parse().unwrap());
    headers.insert("authorization", "Bearer af7bbf6c-4392-3726-90f1-6959447492ba".parse().unwrap());

    let html = client
        .get(format!("https://api.insee.fr/series/BDM/V1/data/SERIES_BDM/{}", id).as_str())
        .headers(headers)
        .send()
        .await.expect("Error 1")
        .text()
        .await.expect("Error 2");
  
    let fragment = Html::parse_fragment(html.as_str());
    let selector = Selector::parse("Obs").unwrap();
  
    let mut time_series = Vec::new();
    for element in fragment.select(&selector) {
  
        let json: Value = element.value().attrs().collect();
        let data_point: TimeSeriesDataPoint = serde_json::from_value(json).expect("json string to Datapoint");
  
        time_series.push(data_point);
      
    };
  
    time_series
  
  }