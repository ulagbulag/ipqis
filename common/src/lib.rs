pub extern crate serde_json;

pub mod json;
pub mod node;

use ipiis_common::{define_io, external_call, Ipiis, ServerResult, CLIENT_DUMMY};
use ipis::{
    async_trait::async_trait,
    core::{
        account::{GuaranteeSigned, GuarantorSigned},
        anyhow::Result,
        data::Data,
    },
    function::DynFunction,
};

use crate::node::NodeTree;

#[async_trait]
pub trait Ipqis {
    async fn protocol(&self) -> Result<String>;

    async fn update_agent(&self, query: DynFunction) -> Result<NodeTree>;
}

#[async_trait]
impl<IpiisClient> Ipqis for IpiisClient
where
    IpiisClient: Ipiis + Send + Sync,
{
    async fn protocol(&self) -> Result<String> {
        // next target
        let target = self.get_account_primary(KIND.as_ref()).await?;

        // external call
        let (protocol,) = external_call!(
            client: self,
            target: KIND.as_ref() => &target,
            request: crate::io => Protocol,
            sign: self.sign_owned(target, ())?,
            inputs: { },
            outputs: { protocol, },
        );

        // unpack response
        Ok(protocol)
    }

    async fn update_agent(&self, query: DynFunction) -> Result<NodeTree> {
        // next target
        let target = self.get_account_primary(KIND.as_ref()).await?;

        // external call
        let (node,) = external_call!(
            client: self,
            target: KIND.as_ref() => &target,
            request: crate::io => UpdateAgent,
            sign: self.sign_owned(target, CLIENT_DUMMY)?,
            inputs: {
                query: query,
            },
            outputs: { node, },
        );

        // unpack response
        Ok(node)
    }
}

define_io! {
    Protocol {
        inputs: { },
        input_sign: Data<GuaranteeSigned, ()>,
        outputs: {
            protocol: String,
        },
        output_sign: Data<GuarantorSigned, ()>,
        generics: { },
    },
    UpdateAgent {
        inputs: {
            query: DynFunction,
        },
        input_sign: Data<GuaranteeSigned, u8>,
        outputs: {
            node: NodeTree,
        },
        output_sign: Data<GuarantorSigned, u8>,
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
