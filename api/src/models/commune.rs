use serde::{Serialize, Deserialize};
use juniper::{FieldResult, graphql_object};
use serde_json::{Value};

use crate::database::Context;
use crate::models::departement::Departement;
 
#[derive(Serialize, Deserialize, Debug)]
pub struct Commune {
    pub nom: String,
    pub code: String,
    #[serde(rename = "codeDepartement")]
    pub code_departement: Option<String>,
    #[serde(rename = "codesPostaux")]
    pub codes_postaux: Vec<String>,
    pub population: Option<i32>,
    pub surface: Option<i32>,
}

#[graphql_object(context = Context)]
impl Commune {

    fn nom(&self) -> &String {
        &self.nom
    } 
    fn code(&self) -> &String {
        &self.code
    } 
    fn population(&self) -> i32 {
        self.population.expect("no pop")
    } 
    fn surface(&self) -> i32 {
        self.surface.expect("no surface")
    } 

    async fn code_departement(&self, context: &Context) -> FieldResult<Departement> {

        let id = self.code_departement.as_ref().expect("departement on commune");

        let data = reqwest::get(format!("https://geo.api.gouv.fr/departements/{}", id).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let departement: Departement = serde_json::from_str(data.as_str()).expect("json-string to departement struct");

        Ok(departement)
        
    }

    ///Opérateur
    async fn operateur(&self, context: &Context) -> FieldResult<String> {
        let data = reqwest::get(
            format!(
                "https://opendata.agenceore.fr/api/records/1.0/search/?dataset=conso-elec-gaz-annuelle-par-secteur-dactivite-agregee-commune&q=&refine.code_commune={}&refine.annee=2019", 
                self.code
            ).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");
            
        let root: Value = serde_json::from_str(&data).expect("deserialize");
        
        let records = root.get("records").expect("no records");
        let fields = records[0].get("fields").expect("key does not exist");
        let label = fields.get("operateur").expect("key does not exist");

        Ok(label.as_str().expect("can't convert to string").to_string())
    }
    
    /// Code Postal
    async fn code_postal(&self, context: &Context) -> FieldResult<String> {
        let data = reqwest::get(
            format!(
                "https://opendata.agenceore.fr/api/records/1.0/search/?dataset=conso-elec-gaz-annuelle-par-secteur-dactivite-agregee-commune&q=&refine.code_commune={}&refine.annee=2019", 
                self.code
            ).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");
            
        let root: Value = serde_json::from_str(&data).expect("deserialize");
        
        let records = root.get("records").expect("no records");
        let fields = records[0].get("fields").expect("key does not exist");
        let label = fields.get("code_postal").expect("key does not exist");

        Ok(label.as_str().expect("can't convert to string").to_string())
    }
    
    /// Consommation totale
    async fn conso_totale(&self, context: &Context) -> FieldResult<Option<f64>> {
        let data = reqwest::get(
            format!(
                "https://opendata.agenceore.fr/api/records/1.0/search/?dataset=conso-elec-gaz-annuelle-par-secteur-dactivite-agregee-commune&q=&refine.code_commune={}&refine.annee=2019", 
                self.code
            ).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");
            
        let root: Value = serde_json::from_str(&data)?;
        
        let records = root.get("records").expect("no records");
        let fields = records[0].get("fields").expect("key does not exist");
        let label = fields.get("consototale").expect("key does not exist");

        Ok(Some(label.as_f64().expect("can't convert to string")))
    }
    
    /// Puissance de raccordement
    async fn sum_3_prod_e_kw_puissance_de_raccordement_injection(&self, context: &Context) -> FieldResult<f64> {
        let data = reqwest::get(
            format!(
                "https://opendata.agenceore.fr/api/records/1.0/search/?dataset=installations-de-production-hydraulique-par-commune&q=&refine.1_f_code_insee_pdl={}", 
                self.code
            ).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");
            
        let root: Value = serde_json::from_str(&data).expect("deserialize");
        
        let records = root.get("records").expect("no records");
        let fields = records[0].get("fields").expect("key does not exist");
        let label = fields.get("sum_3_prod_e_kw_puissance_de_raccordement_injection").expect("key does not exist");

        Ok(label.as_f64().expect("can't convert to string"))
    }

}