use hyper::{
    body::{self, Bytes},
    Body, Request,
};
use serde::Serialize;

use crate::{
    model::{ApiResult, Availability, Metadata, MotorType},
    Error, InnerClient, API_URL,
};

macro_rules! by {
    ($name:ident($input:ident: $type:ty)) => {
        paste::paste! {
            pub fn [<by_ $name>](mut self, $input: $type) -> Self {
                self.$input.replace($input.to_owned());

                self
            }
        }
    };
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    impulse_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diameter: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    ty: Option<MotorType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_org: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability: Option<Availability>,
    #[serde(skip)]
    client: InnerClient,
}

impl MetadataBuilder {
    by!(manufacturer(manufacturer: &str));
    by!(impulse_class(impulse_class: &str));
    by!(diameter(diameter: f32));
    by!(motor_type(ty: MotorType));
    by!(cert_org(cert_org: &str));
    by!(availability(availability: Availability));

    pub(crate) fn new(client: InnerClient) -> Self {
        Self {
            client,
            availability: None,
            cert_org: None,
            diameter: None,
            impulse_class: None,
            manufacturer: None,
            ty: None,
        }
    }

    pub async fn get(self) -> Result<Metadata, Error> {
        let req = Request::builder()
            .method("POST")
            .header("Content-Type", "application/json")
            .uri(format!("{}/metadata.json", API_URL))
            .body(Body::from(serde_json::to_vec(&self).unwrap()))?;

        let response = self.client.request(req).await?;

        if !response.status().is_success() {
            return Err(Error::Status {
                code: response.status(),
            });
        }

        let body: Bytes = body::to_bytes(response.into_body()).await?;

        let value: ApiResult<Metadata> = serde_json::from_slice(&body[..])?;

        dbg!(value);

        todo!()
    }
}
