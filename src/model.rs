use mongodb::bson::{oid::ObjectId, serde_helpers::*};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    #[serde(
        rename = "_id",
        serialize_with = "serialize_hex_string_as_object_id",
        deserialize_with = "deserialize_object_id_to_hex_string"
    )]
    pub id: String,
    pub name: String,
    pub url: String,
    pub interval: i16,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertableJob {
    pub name: String,
    pub url: String,
    pub interval: i16,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Filter {
    CSSFilter(CSSFilterOptions),
    XPathFilter(XPathFilterOptions),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSSFilterOptions {
    pub selector: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XPathFilterOptions {
    pub selector: String,
}

pub fn deserialize_object_id_to_hex_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let object_id = ObjectId::deserialize(deserializer)?;

    Ok(object_id.to_hex())
}
