pub extern crate serde_json;

use core::str::FromStr;
use std::collections::HashMap;

use ipis::{
    attention::AttentionUnit,
    core::{
        anyhow::bail,
        value::{
            hash::Hash,
            text::{Text, TextHash},
        },
    },
};
use serde_json::Number;

#[derive(Clone)]
pub struct Key {
    pub text: Text,
    pub hash: TextHash,
    pub kind: Kind,
}

impl ::core::borrow::Borrow<str> for Key {
    fn borrow(&self) -> &str {
        self.text.msg.as_str()
    }
}

impl ::core::fmt::Debug for Key {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.text.msg, f)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.text.msg.as_str(), other.text.msg.as_str())
    }
}

impl Eq for Key {}

impl ::core::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ::core::hash::Hash::hash(self.text.msg.as_str(), state)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueCandidate {
    pub attention: AttentionUnit,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    Text(Text),
    Array { len: u64 },
    Object,
}

impl Value {
    pub fn unwrap_text(&self) -> &Text {
        match self {
            Self::Text(v) => v,
            _ => panic!("failed to unwrap value as Text: {self:?}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Null,
    Bool,
    Number,
    Text,
    Array,
    Object,
}

impl Kind {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            Self::Null => "Null",
            Self::Bool => "Bool",
            Self::Number => "Number",
            Self::Text => "Text",
            Self::Array => "Array",
            Self::Object => "Object",
        }
    }
}

impl FromStr for Kind {
    type Err = ::ipis::core::anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Null" => Ok(Self::Null),
            "Bool" => Ok(Self::Bool),
            "Number" => Ok(Self::Number),
            "Text" => Ok(Self::Text),
            "Array" => Ok(Self::Array),
            "Object" => Ok(Self::Object),
            _ => bail!("failed to parse the kind: {s}"),
        }
    }
}

impl From<Kind> for Hash {
    fn from(value: Kind) -> Self {
        Hash::with_str(value.as_static_str())
    }
}

impl TryFrom<Hash> for Kind {
    type Error = ::ipis::core::anyhow::Error;

    fn try_from(value: Hash) -> Result<Self, Self::Error> {
        match value {
            value if value == Hash::with_str("Null") => Ok(Self::Null),
            value if value == Hash::with_str("Bool") => Ok(Self::Bool),
            value if value == Hash::with_str("Number") => Ok(Self::Number),
            value if value == Hash::with_str("Text") => Ok(Self::Text),
            value if value == Hash::with_str("Array") => Ok(Self::Array),
            value if value == Hash::with_str("Object") => Ok(Self::Object),
            _ => bail!("failed to parse the kind: {value:?}"),
        }
    }
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        self.as_static_str().into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeTree {
    pub value: ValueCandidate,
    pub children: HashMap<Key, Self>,
}
