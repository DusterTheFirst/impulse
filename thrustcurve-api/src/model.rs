use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Error { error: String },
    Response(T),
    Empty {},
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub manufacturers: Vec<NameAndAbbrev>,
    pub cert_orgs: Vec<NameAndAbbrev>,
    pub types: Vec<MotorType>,
    pub diameters: Vec<f32>,
    pub impulse_classes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MotorType {
    #[serde(rename = "SU")]
    SingleUse,
    Reload,
    Hybrid,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Availability {
    Available,
    #[serde(rename = "OOP")]
    OutOfProduction,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameAndAbbrev {
    pub name: String,
    pub abbrev: String,
}
