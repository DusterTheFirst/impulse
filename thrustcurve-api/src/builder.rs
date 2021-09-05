use crate::model::{Availability, Metadata, MotorType};

macro_rules! by {
    ($name:ident($input:ident: $type:ty)) => {
        paste::paste! {
            pub fn [<by_ $name>](mut self, $input: $type) -> Self {
                self.$input.replace($input);

                self
            }
        }
    };
}

#[derive(Default)]
pub struct MetadataBuilder {
    pub manufacturer: Option<String>,
    pub impulse_class: Option<String>,
    pub diameter: Option<f32>,
    pub ty: Option<MotorType>,
    pub cert_org: Option<String>,
    pub availability: Option<Availability>,
}

impl MetadataBuilder {
    by!(manufacturer(manufacturer: String));
    by!(impulse_class(impulse_class: String));
    by!(diameter(diameter: f32));
    by!(motor_type(ty: MotorType));
    by!(cert_org(cert_org: String));
    by!(availability(availability: Availability));

    pub async fn get(self) -> Metadata {
        todo!()
    }
}
