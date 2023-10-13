use serde::Serialize;

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
