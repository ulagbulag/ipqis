pub mod json;
pub mod node;

use ipiis_common::{define_io, external_call, Ipiis, ServerResult};
use ipis::{
    async_trait::async_trait,
    core::{
        account::{GuaranteeSigned, GuarantorSigned},
        anyhow::Result,
    },
    function::{DynFunction, Function},
};

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

        // external call
        let (functions,) = external_call!(
            client: self,
            target: KIND.as_ref() => &target,
            request: crate::io => SearchFunctions,
            sign: self.sign(target, ())?,
            inputs: {
                query: query,
            },
            outputs: { functions, },
        );

        // unpack response
        Ok(functions)
    }
}

define_io! {
    SearchFunctions {
        inputs: {
            query: DynFunction,
        },
        input_sign: GuaranteeSigned<()>,
        outputs: {
            functions: Vec<Function>,
        },
        output_sign: GuarantorSigned<()>,
        generics: { },
    },
}

::ipis::lazy_static::lazy_static! {
    pub static ref KIND: Option<::ipis::core::value::hash::Hash> = Some(
        ::ipis::core::value::hash::Hash::with_str("__ipis__ipqis__"),
    );

    pub static ref KIND_KEY: ::ipis::core::value::hash::Hash =
        ::ipis::core::value::hash::Hash::with_str("__ipis__ipqis__key__");
    pub static ref KIND_VALUE: ::ipis::core::value::hash::Hash =
        ::ipis::core::value::hash::Hash::with_str("__ipis__ipqis__value__");
}
