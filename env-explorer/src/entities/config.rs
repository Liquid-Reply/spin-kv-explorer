use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct KeyValue {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct File {
    pub(crate) path: String,
}

// #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
// pub(crate) struct Config {
//     pub(crate) envvars: Vec::<KeyValue>,
//     // pub(crate) config: Vec::<KeyValue>,
//     pub(crate) files: Vec::<String>,
//     // pub(crate) secrets: Vec::<KeyValue>,
//     // pub(crate) redisKV: Vec::<KeyValue>,
// }
