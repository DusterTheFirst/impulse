use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub manufacturers: Vec<NameAndAbbrev>,
    pub cert_orgs: Vec<NameAndAbbrev>,
    pub types: Vec<MotorType>,
    pub diameters: Vec<f32>,
    pub impulse_classes: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MotorType {
    #[serde(rename = "SU")]
    SingleUse,
    Reload,
    Hybrid,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Availability {
    Available,
    #[serde(rename = "OOP")]
    OutOfProduction,
}

#[derive(Deserialize)]
pub struct NameAndAbbrev {
    pub name: String,
    pub abbrev: String,
}
