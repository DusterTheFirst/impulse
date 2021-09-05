#![deny(unsafe_code)]

use builder::MetadataBuilder;
use hyper::client::HttpConnector;

mod builder;
mod model;

#[derive(Default)]
pub struct Client(hyper::Client<HttpConnector>);

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn metadata() -> MetadataBuilder {
        MetadataBuilder::default()
    }
}
