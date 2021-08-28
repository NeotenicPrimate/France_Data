use juniper::{graphql_object, FieldResult};

use crate::database::Context;
use crate::models::{region::Region, commune::Commune, departement::Departement, country::Country};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {

    fn apiVersion() -> String {
        "0.0".to_string()
    }

    async fn region(id: String) -> FieldResult<Region> {

        let data = reqwest::get(format!("https://geo.api.gouv.fr/regions/{}", id).as_str())
            .await?
            .text()
            .await?;

        let region: Region = serde_json::from_str(data.as_str()).expect("json string to region struct");

        Ok(region)

    }
    
    async fn regions() -> FieldResult<Vec<Region>> {

        let data = reqwest::get("https://geo.api.gouv.fr/regions")
            .await?
            .text()
            .await?;

        let regions: Vec<Region> = serde_json::from_str(data.as_str())?;

        Ok(regions)

    }

    async fn communes() -> FieldResult<Vec<Commune>> {
        let data = reqwest::get("https://geo.api.gouv.fr/communes")
            .await?
            .text()
            .await?;

        let communes: Vec<Commune> = serde_json::from_str(data.as_str())?;

        Ok(communes)
    }

    async fn commune(id: String) -> FieldResult<Commune> {
        let data = reqwest::get(format!("https://geo.api.gouv.fr/communes/{}", id).as_str())
            .await?
            .text()
            .await?;

        let commune: Commune = serde_json::from_str(data.as_str())?;

        Ok(commune)
    }

    async fn departement(id: String) -> FieldResult<Departement> {
        let data = reqwest::get(format!("https://geo.api.gouv.fr/departements/{}", id).as_str())
            .await?
            .text()
            .await?;

        let departement: Departement = serde_json::from_str(data.as_str())?;

        Ok(departement)
    }
  
    async fn departements() -> FieldResult<Vec<Departement>> {
        let data = reqwest::get("https://geo.api.gouv.fr/departements")
            .await?
            .text()
            .await?;

        let departements: Vec<Departement> = serde_json::from_str(data.as_str())?;

        Ok(departements)
    }

    async fn country(id: String) -> FieldResult<Country> {
        let data = reqwest::get(format!("https://restcountries.eu/rest/v2/alpha/{}", id).as_str())
            .await?
            .text()
            .await?;

        let country: Country = serde_json::from_str(data.as_str())?;

        Ok(country)
    }
    
    async fn countries() -> FieldResult<Vec<Country>> {
        let data = reqwest::get("https://restcountries.eu/rest/v2/all")
            .await?
            .text()
            .await?;

        let countries: Vec<Country> = serde_json::from_str(data.as_str())?;

        Ok(countries)
    }


}