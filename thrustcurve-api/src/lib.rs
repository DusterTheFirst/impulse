#![deny(unsafe_code)]

use hyper::client::HttpConnector;
use serde::Deserialize;

pub struct Client(hyper::Client<HttpConnector>);

impl Client {
    pub fn new() -> Self {
        Self(hyper::Client::new())
    }

    pub fn metadata() -> MetadataBuilder {}
}

pub struct MetadataBuilder {
    pub manufacturer: Option<String>,
    pub impulseClass: Option<String>,
    pub diameter: Option<f32>,
    pub ty: Option<MotorType>,
    pub certOrg: Option<String>,
    pub availability: Option<Availability>,
}

impl MetadataBuilder {
    pub fn by_manufacturer(self, name_or_abbr: String) -> Self {
        self.manufacturer.replace(name_or_abbr);

        self
    }

    pub async fn get(self) -> Metadata {
        todo!()
    }
}

struct Metadata {
    pub manufacturers: Vec<NameAndAbbrev>,
    pub certOrgs: Vec<NameAndAbbrev>,
    pub types: Vec<MotorType>,
    pub diameters: Vec<f32>,
    pub impulseClasses: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum MotorType {
    #[serde(rename = "SU")]
    SingleUse,
    Reload,
    Hybrid,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Availability {
    Available,
    #[serde(rename = "OOP")]
    OutOfProduction,
}

struct NameAndAbbrev {
    pub name: String,
    pub abbrev: String,
}
