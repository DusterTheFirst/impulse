#![deny(unsafe_code)]

use builder::MetadataBuilder;
use hyper::{body, client::HttpConnector, Body, Request, StatusCode};
use hyper_tls::HttpsConnector;
use model::ApiResult;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

mod builder;
mod model;

pub const API_URL: &str = "https://www.thrustcurve.org/api/v1";

/// Wraps an inner [`hyper::Client`] and provides access to useful builders
/// to compose requests to <https://thrustcurve.org>
pub struct Client {
    inner: InnerClient,
}

pub(crate) type InnerClient = hyper::Client<HttpsConnector<HttpConnector>>;

impl Client {
    /// Create an api client using default settings and a default hyper client
    pub fn new() -> Self {
        Self {
            inner: hyper::Client::builder().build(HttpsConnector::new()),
        }
    }

    /// Create an api client backed by a customized hyper client
    pub fn new_with_client(client: InnerClient) -> Self {
        Self { inner: client }
    }

    /// Get metadata about all motors in the database.
    pub fn metadata(&self) -> MetadataBuilder {
        MetadataBuilder::new(self.inner.clone())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// All encompassing error type returned from any api request
#[derive(Debug, Error)]
pub enum Error {
    /// Failure returned from the underlying hyper client
    #[error("failed to make api request: {0}")]
    Request(#[from] hyper::Error),
    /// Failure in creating a hyper request
    #[error("failed to construct an api request: {0}")]
    Http(#[from] hyper::http::Error),
    /// Failure returned from serde_json
    #[error("failed to (de)serialize json data: {0}")]
    Json(#[from] serde_json::Error),
    /// Failure returned from the API endpoint in the form of JSON
    #[error("api returned an error: {message}")]
    Api { message: String },
    /// Failure returned from the API endpoint in the form of a non 2xx status code
    #[error("api endpoint returned an unsuccessful status code: {code}")]
    Status { code: StatusCode },
}

/// Utility function to fetch an endpoint from the thrustcurve API
pub(crate) async fn get_endpoint<B: Serialize, R: DeserializeOwned>(
    client: InnerClient,
    body: B,
    path: &str,
) -> Result<Option<R>, Error> {
    let req = Request::builder()
        .method("POST")
        .header("Content-Type", "application/json")
        .uri(format!("{}/{}", API_URL, path))
        .body(Body::from(serde_json::to_vec(&body)?))?;

    let response = client.request(req).await?;

    if !response.status().is_success() {
        return Err(Error::Status {
            code: response.status(),
        });
    }

    let response = body::to_bytes(response.into_body()).await?;

    match serde_json::from_slice(&response[..])? {
        ApiResult::Error { error } => Err(Error::Api { message: error }),
        ApiResult::Response(r) => Ok(Some(r)),
        ApiResult::Empty {} => Ok(None),
    }
}
