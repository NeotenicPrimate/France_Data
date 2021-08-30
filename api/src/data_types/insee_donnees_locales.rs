// use serde::{Serialize, Deserialize};
// use juniper::{GraphQLObject};
// use reqwest::header::{HeaderMap};
// use scraper::{Html, Selector};
// use serde_json::{Value};



// pub async fn request_insee_donnees_locales(
//     donnee: &str,
//     croisement: &str,
//     jeu_donnees: &str,
//     nivgeo: &str,
//     codgeo: &str,
//     modalite: &str,
// ) -> Vec<TimeSeriesDataPoint> {
//     let client = reqwest::Client::new();
  
//     let mut headers = HeaderMap::new();
//     headers.insert("accept", "application/xml".parse().unwrap());
//     headers.insert("authorization", "Bearer af7bbf6c-4392-3726-90f1-6959447492ba".parse().unwrap());

//     let html = client
//         .get(format!("https://api.insee.fr/donnees-locales/V0.1/{}/geo-{}@{}/{}-{}.{}", 
//             donnee, 
//             croisement, 
//             jeu_donnees, 
//             nivgeo, 
//             codgeo, 
//             modalite
//         ).as_str())
//         .headers(headers)
//         .send()
//         .await.expect("Error 1")
//         .text()
//         .await.expect("Error 2");
  
//     let fragment = Html::parse_fragment(html.as_str());
//     let selector = Selector::parse("Obs").unwrap();
  
//     let mut time_series = Vec::new();
//     for element in fragment.select(&selector) {
  
//         let json: Value = element.value().attrs().collect();
//         let data_point: TimeSeriesDataPoint = serde_json::from_value(json).expect("json string to Datapoint");
  
//         time_series.push(data_point);
      
//     };
  
//     time_series
  
//   }