use core::str::FromStr;
use std::collections::BTreeMap;

use bytecheck::CheckBytes;
use ipis::{
    attention::AttentionUnit,
    core::{
        anyhow::bail,
        signed::IsSigned,
        value::{
            hash::Hash,
            text::{Text, TextHash},
        },
    },
};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct Key {
    pub text: Text,
    pub hash: TextHash,
    pub kind: Kind,
}

impl IsSigned for Key {}

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

impl PartialEq for ArchivedKey {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.text.msg, &other.text.msg)
    }
}

impl Eq for Key {}

impl Eq for ArchivedKey {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(self.text.msg.as_str(), other.text.msg.as_str())
    }
}

impl PartialOrd for ArchivedKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.text.msg, &other.text.msg)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(self.text.msg.as_str(), other.text.msg.as_str())
    }
}

impl Ord for ArchivedKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&self.text.msg, &other.text.msg)
    }
}

impl ::core::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ::core::hash::Hash::hash(self.text.msg.as_str(), state)
    }
}

impl ::core::hash::Hash for ArchivedKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ::core::hash::Hash::hash(&self.text.msg, state)
    }
}

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct ValueCandidate {
    pub attention: AttentionUnit,
    pub value: Value,
}

impl IsSigned for ValueCandidate {}

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub enum Value {
    Null,
    Bool(bool),
    U64(u64),
    I64(i64),
    F64(f64),
    Text(Text),
    Array { len: u64 },
    Object,
}

impl IsSigned for Value {}

impl Value {
    pub fn unwrap_text(&self) -> &Text {
        match self {
            Self::Text(v) => v,
            _ => panic!("failed to unwrap value as Text: {self:?}"),
        }
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub enum Kind {
    Null,
    Bool,
    I64,
    U64,
    F64,
    Text,
    Array,
    Object,
}

impl IsSigned for Kind {}

impl Kind {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            Self::Null => "Null",
            Self::Bool => "Bool",
            Self::I64 => "I64",
            Self::U64 => "U64",
            Self::F64 => "F64",
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
            "I64" => Ok(Self::I64),
            "U64" => Ok(Self::U64),
            "F64" => Ok(Self::F64),
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
            value if value == Hash::with_str("I64") => Ok(Self::I64),
            value if value == Hash::with_str("U64") => Ok(Self::U64),
            value if value == Hash::with_str("F64") => Ok(Self::F64),
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

#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
#[archive_attr(derive(CheckBytes, Debug, PartialEq))]
pub struct NodeTree {
    pub children: BTreeMap<Key, ValueCandidate>,
}

impl IsSigned for NodeTree {}
