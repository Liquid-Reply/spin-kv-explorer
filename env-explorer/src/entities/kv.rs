use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct Entry {
    pub(crate) store: String,
    pub(crate) key: String,
    pub(crate) value: String,
}
