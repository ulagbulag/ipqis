#![feature(more_qualified_paths)]

use bytecheck::CheckBytes;
use ipiis_common::{external_call, Ipiis};
use ipis::{
    async_trait::async_trait,
    core::{account::GuaranteeSigned, anyhow::Result},
    function::{DynFunction, Function},
};
use rkyv::{Archive, Deserialize, Serialize};

#[async_trait]
pub trait Ipqis {
    async fn search_functions(&self, query: DynFunction) -> Result<Vec<Function>>;
}

#[async_trait]
impl<IpiisClient> Ipqis for IpiisClient
where
    IpiisClient: Ipiis + Send + Sync,
{
    async fn search_functions(&self, query: DynFunction) -> Result<Vec<Function>> {
        // next target
        let target = self.get_account_primary(KIND.as_ref()).await?;

        // pack request
        let req = RequestType::SearchFunctions { query };

        // external call
        let (functions,) = external_call!(
            call: self
                .call_permanent_deserialized(&target, req)
                .await?,
            response: Response => SearchFunctions,
            items: { functions },
        );

        // unpack response
        Ok(functions)
    }
}

pub type Request = GuaranteeSigned<RequestType>;

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub enum RequestType {
    SearchFunctions { query: DynFunction },
}

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub enum Response {
    SearchFunctions { functions: Vec<Function> },
}

::ipis::lazy_static::lazy_static! {
    pub static ref KIND: Option<::ipis::core::value::hash::Hash> = Some(
        ::ipis::core::value::hash::Hash::with_str("__ipis__ipqis__"),
    );
}
