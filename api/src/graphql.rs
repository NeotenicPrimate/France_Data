use juniper::{RootNode, EmptyMutation, EmptySubscription, graphql_object};

use crate::database::Context;
use crate::models::{region::Region, commune::Commune, departement::Departement, country::Country};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {

    fn apiVersion() -> String {
        "1.0".to_string()
    }

    async fn region(id: String) -> Region {

        let data = reqwest::get(format!("https://geo.api.gouv.fr/regions/{}", id).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let region: Region = serde_json::from_str(data.as_str()).expect("json string to region struct");

        region

    }

    async fn communes() -> Vec<Commune> {
        let data = reqwest::get("https://geo.api.gouv.fr/communes")
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let communes: Vec<Commune> = serde_json::from_str(data.as_str()).expect("json string to commune struct");

        communes
    }

    async fn commune(id: String) -> Commune {
        let data = reqwest::get(format!("https://geo.api.gouv.fr/communes/{}", id).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let commune: Commune = serde_json::from_str(data.as_str()).expect("json string to commune struct");

        commune
    }

    async fn departement(id: String) -> Departement {
        let data = reqwest::get(format!("https://geo.api.gouv.fr/departements/{}", id).as_str())
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let departement: Departement = serde_json::from_str(data.as_str()).expect("json string to commune struct");

        departement
    }

    async fn country() -> Country {
        let data = reqwest::get("https://restcountries.eu/rest/v2/alpha/FRA")
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let country: Country = serde_json::from_str(data.as_str()).expect("json string to country struct");

        country
    }
    
    async fn countries() -> Vec<Country> {
        let data = reqwest::get("https://restcountries.eu/rest/v2/all")
            .await.expect("Error")
            .text()
            .await.expect("Error");

        let countries: Vec<Country> = serde_json::from_str(data.as_str()).expect("json string to country struct");

        countries
    }


}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}