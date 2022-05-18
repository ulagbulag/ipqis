use bytecheck::CheckBytes;
use ipiis_common::Ipiis;
use ipis::{async_trait::async_trait, core::account::GuaranteeSigned};
use rkyv::{Archive, Deserialize, Serialize};

#[async_trait]
pub trait Ipqis {}

#[async_trait]
impl<IpiisClient> Ipqis for IpiisClient where IpiisClient: Ipiis + Send + Sync {}

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

::ipis::lazy_static::lazy_static! {
    pub static ref KIND: Option<::ipis::core::value::hash::Hash> = Some(
        ::ipis::core::value::hash::Hash::with_str("__ipis__ipqis__"),
    );
}
