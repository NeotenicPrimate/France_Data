use serde::{Serialize, Deserialize};
use juniper::{FieldResult, graphql_object};
use serde_json::{Value};

use crate::database::Context;
use crate::models::{departement::Departement, country::Country};

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
  pub nom: String,
  pub code: String,
}

#[graphql_object(context = Context)]
impl Region {

    fn nom(&self) -> &String {
        &self.nom
    }

    fn code(&self) -> &String {
        &self.code
    }

    async fn code_country(&self, context: &Context) -> FieldResult<Country> {

        let data = reqwest::get("https://restcountries.eu/rest/v2/alpha/FRA")
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let country: Country = serde_json::from_str(data.as_str()).expect("json-string to departement struct");

        Ok(country)

    }

    async fn code_departements(&self, context: &Context) -> FieldResult<Vec<Departement>> {

        let data = reqwest::get(format!("https://geo.api.gouv.fr/regions/{}/departements", self.code).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let departements: Vec<Departement> = serde_json::from_str(data.as_str()).expect("json-string to departement struct");

        Ok(departements)

    }

    /// Consommation totale par région
    async fn conso_totale(&self, context: &Context) -> FieldResult<f64> {
        let data = reqwest::get(
            format!(
                "https://opendata.agenceore.fr/api/records/1.0/search/?dataset=conso-elec-gaz-annuelle-par-secteur-dactivite-agregee-region&q=&refine.code_region={}&refine.annee=2019", 
                self.code
            ).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");
            
        let root: Value = serde_json::from_str(&data).expect("deserialize");
        
        let records = root.get("records").expect("no records");
        let fields = records[0].get("fields").expect("key does not exist");
        let label = fields.get("consototale").expect("key does not exist");

        Ok(label.as_f64().expect("can't convert to string"))
    }

}