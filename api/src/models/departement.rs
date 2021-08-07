use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject, FieldResult, graphql_object};
use serde_json::{Value};

use crate::database::Context;
use crate::models::{region::Region, commune::Commune};

#[derive(Serialize, Deserialize, Debug)]
pub struct Departement {
  pub nom: String,
  pub code: String,
  #[serde(rename = "codeRegion")]
  pub code_region: Option<String>,
}

#[graphql_object(context = Context)]
impl Departement {

    fn nom(&self) -> &String {
        &self.nom
    }
    
    fn code(&self) -> &String {
        &self.code
    }

    async fn code_region(&self, context: &Context) -> FieldResult<Option<Region>> {

        let id = self.code_region.as_ref();

        let id = match id {
            Some(value) => value,
            None => return Ok(None),
        };

        let data = reqwest::get(format!("https://geo.api.gouv.fr/regions/{}", id).as_str())
            .await?
            .text()
            .await?;

        let region: Region = serde_json::from_str(data.as_str())?;

        Ok(Some(region))

    }
    
    async fn code_communes(&self, context: &Context) -> FieldResult<Option<Vec<Commune>>> {

        let data = reqwest::get(format!("https://geo.api.gouv.fr/departements/{}/communes?fields=nom,code,codesPostaux,codeDepartement,codeRegion,population&format=json&geometry=centre", 
            self.code).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let communes: Vec<Commune> = serde_json::from_str(data.as_str())?;

        Ok(Some(communes))

    }

    async fn conso_totale(&self, context: &Context) -> FieldResult<f64> {
        let data = reqwest::get(
            format!(
                "https://opendata.agenceore.fr/api/records/1.0/search/?dataset=conso-elec-gaz-annuelle-par-secteur-dactivite-agregee-region&q=&refine.code_departement={}&refine.annee=2019", 
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