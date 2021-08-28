use juniper::{graphql_object, FieldResult};
use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap};
use scraper::{Html, Selector};
use serde_json::{Value};

use crate::database::Context;
use crate::models::{region::Region};
use crate::data_types::{time_series::{TimeSeriesDataPoint, request_insee_bdm}};

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
        self.population
    }

    async fn gini(&self) -> Option<f64> {
        self.gini
    }

    /// Personnes dans le halo autour du chômage - Inactifs faisant des démarches actives de recherche d'emploi mais non disponibles (en milliers) - France hors Mayotte - Données CVS
    /// INSEE Identifiant 010605048
    async fn inactive_cherchant_emploi(&self) -> FieldResult<Option<Vec<TimeSeriesDataPoint>>> {
          
        let time_series = request_insee_bdm("010605048").await;

        Ok(Some(time_series))

    }

    /// Situation mensuelle du budget de l'État - Solde général d'exécution - Cumul depuis le début de l'année
    /// INSEE Identifiant 001717255
    async fn solde_general_dexectution_mensuel_etat() -> FieldResult<Option<Vec<TimeSeriesDataPoint>>> { 

        let time_series = request_insee_bdm("001717255").await;
        
        Ok(Some(time_series))
        
    }

    /// Situation mensuelle du budget de l'État - Dépenses - Cumul depuis le début de l'année
    /// Identifiant 001717256
    async fn depenses_mensuelles_etat() -> FieldResult<Option<Vec<TimeSeriesDataPoint>>> {

        let time_series = request_insee_bdm("001717256").await;
        
        Ok(Some(time_series))

    }

    // TODO: find link between Countries and their Regions
    /// 
    async fn code_regions() -> FieldResult<Option<Vec<Region>>> {

        let data = reqwest::get("https://geo.api.gouv.fr/regions")
            .await?
            .text()
            .await?;

        let regions: Vec<Region> = serde_json::from_str(data.as_str())?;

        Ok(Some(regions))

    }
}