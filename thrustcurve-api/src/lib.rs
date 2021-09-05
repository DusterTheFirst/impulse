#![deny(unsafe_code)]

use builder::MetadataBuilder;
use hyper::{client::HttpConnector, StatusCode};
use hyper_tls::HttpsConnector;
use thiserror::Error;

mod builder;
mod model;

pub const API_URL: &str = "https://www.thrustcurve.org/api/v1";

pub struct Client {
    inner: InnerClient,
}

pub(crate) type InnerClient = hyper::Client<HttpsConnector<HttpConnector>>;

impl Client {
    pub fn new() -> Self {
        Self {
            inner: hyper::Client::builder().build(HttpsConnector::new()),
        }
    }

    pub fn new_with_client(client: InnerClient) -> Self {
        Self { inner: client }
    }

    pub fn metadata(&self) -> MetadataBuilder {
        MetadataBuilder::new(self.inner.clone())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to make api request: {0}")]
    Request(#[from] hyper::Error),
    #[error("failed to construct an api request: {0}")]
    Http(#[from] hyper::http::Error),
    #[error("failed to deserialize incoming json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("api returned an error: {message}")]
    Api { message: String },
    #[error("api endpoint returned an unsuccessful status code: {code}")]
    Status { code: StatusCode },
}

#[cfg(test)]
mod test {
    use crate::{
        model::{Availability, MotorType},
        Client, Error,
    };

    #[tokio::test]
    async fn metadata() -> Result<(), Error> {
        let client = Client::new();

        let result = client
            .metadata()
            .by_availability(Availability::Available)
            .by_motor_type(MotorType::SingleUse)
            .by_manufacturer("AMW")
            .get()
            .await?;

        // TODO: MOCK API SOMEHOW

        Ok(())
    }
}
