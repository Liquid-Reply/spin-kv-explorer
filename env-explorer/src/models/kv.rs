use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct AllKeysListModel {
    pub store: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct EntryModel {
    pub store: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateEntryModel {
    pub store: String,
    pub key: String,
    pub value: String,
}

impl TryFrom<&Option<Bytes>> for CreateEntryModel {
    type Error = anyhow::Error;

    fn try_from(value: &Option<Bytes>) -> Result<Self, Self::Error> {
        match value {
            Some(b) => Ok(serde_json::from_slice::<CreateEntryModel>(b)?),
            None => Err(anyhow::anyhow!("No body")),
        }
    }
}