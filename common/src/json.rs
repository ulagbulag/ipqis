use ipdis_common::Ipdis;
use ipiis_common::Ipiis;
use ipis::{
    async_recursion::async_recursion,
    core::{
        anyhow::Result,
        value::{hash::Hash, text::Text},
    },
    futures,
    path::Path,
    word::{WordHash, WordKeyHash},
};
use ipsis_common::Ipsis;

use crate::{node::Kind, KIND_KEY, KIND_VALUE};

#[async_recursion]
pub async fn dump_raw<IpiisClient>(
    client: &IpiisClient,
    parent: Option<&'async_recursion str>,
    key: &str,
    value: &::serde_json::Value,
) -> Result<()>
where
    IpiisClient: Ipdis + Ipiis + Ipsis + Sync,
{
    async fn json_dump_value_primitive(
        client: &(impl Ipdis + Ipiis + Ipsis + Sync),
        parent: &str,
        key: &str,
        kind: Kind,
        value_path: Path,
    ) -> Result<()> {
        let key = if parent.is_empty() {
            key.to_string()
        } else {
            format!("{parent}.{key}")
        };
        let key = Text::with_en_us(&key);
        let key_path = client.put(&key).await?;
        let key_hash = key.into();

        let key_word = WordHash {
            key: WordKeyHash {
                namespace: *KIND_KEY,
                text: key_hash,
            },
            kind: kind.into(),
            relpath: false,
            path: key_path,
        };
        let key_word = client.sign(client.account_me().account_ref(), key_word)?;

        let value_word = WordHash {
            key: WordKeyHash {
                namespace: *KIND_VALUE,
                text: key_hash,
            },
            kind: kind.into(),
            relpath: false,
            path: value_path,
        };
        let value_word = client.sign(client.account_me().account_ref(), value_word)?;

        let parent_hash = Hash::with_str(parent);
        client.put_word(&parent_hash, &key_word).await?;
        client.put_word(&parent_hash, &value_word).await?;
        Ok(())
    }

    match value {
        ::serde_json::Value::Null => {
            let parent = parent.unwrap_or(key);
            let kind = Kind::Null;
            let path = client.put(&()).await?;
            json_dump_value_primitive(client, parent, key, kind, path).await
        }
        ::serde_json::Value::Bool(value) => {
            let parent = parent.unwrap_or(key);
            let kind = Kind::Bool;
            let path = client.put(value).await?;
            json_dump_value_primitive(client, parent, key, kind, path).await
        }
        ::serde_json::Value::Number(value) => {
            let parent = parent.unwrap_or(key);
            let kind = Kind::Number;
            let path = client.put(&value.to_string()).await?;
            json_dump_value_primitive(client, parent, key, kind, path).await
        }
        ::serde_json::Value::String(value) => {
            let parent = parent.unwrap_or(key);
            let kind = Kind::Text;
            let path = client.put(&Text::with_en_us(value)).await?;
            json_dump_value_primitive(client, parent, key, kind, path).await
        }
        ::serde_json::Value::Array(_values) => {
            todo!("Array is not supported yet.");
            // let parent = parent.unwrap_or(key);
            // for value in values {
            //     json_dump_value(client, parent, key, value).await?
            // }
        }
        ::serde_json::Value::Object(values) => {
            if let Some(parent) = parent {
                let kind = Kind::Object;
                let path = client.put(&()).await?;
                json_dump_value_primitive(client, parent, key, kind, path).await?;
            }

            let parent = match parent {
                Some(parent) if !parent.is_empty() => format!("{parent}.{key}"),
                _ => key.to_string(),
            };

            futures::future::try_join_all(
                values
                    .into_iter()
                    .map(|(key, value)| dump_raw(client, Some(&parent), key, value)),
            )
            .await
            .map(|_| ())
        }
    }
}
