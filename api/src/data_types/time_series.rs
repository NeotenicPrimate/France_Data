use serde::{Serialize, Deserialize};
use juniper::{GraphQLObject};

#[derive(Serialize, Deserialize, GraphQLObject, Debug)]
pub struct TimeSeriesDataPoint {
    OBS_QUAL: Option<String>,
    OBS_STATUS: Option<String>,
    OBS_TYPE: Option<String>,
    OBS_VALUE: Option<String>,
    TIME_PERIOD: Option<String>,
}