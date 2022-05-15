use bytecheck::CheckBytes;
use ipiis_api::client::IpiisClient;
use ipis::{async_trait::async_trait, core::account::GuaranteeSigned};
use rkyv::{Archive, Deserialize, Serialize};

#[async_trait]
pub trait Ipqis {}

#[async_trait]
impl Ipqis for IpiisClient {}

pub type Request = GuaranteeSigned<RequestType>;

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub enum RequestType {
    TODO,
}

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub enum Response {
    TODO,
}
