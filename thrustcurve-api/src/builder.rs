use serde::Serialize;

use crate::{
    get_endpoint,
    model::{Availability, Metadata, MotorType},
    Error, InnerClient,
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

    pub async fn get(self) -> Result<Option<Metadata>, Error> {
        get_endpoint(self.client.clone(), &self, "metadata.json").await
    }
}
