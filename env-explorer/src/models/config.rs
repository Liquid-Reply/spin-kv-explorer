use serde::Serialize;

use crate::entities::{File, KeyValue};

#[derive(Debug, Serialize)]
pub(crate) struct EnvVarListModel {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct ConValListModel {
    pub key: String,
    pub value: String,
}

impl From<KeyValue> for EnvVarListModel {
    fn from(value: KeyValue) -> Self {
        Self {
            key: value.key,
            value: value.value,
        }
    }
}

impl From<KeyValue> for ConValListModel {
    fn from(value: KeyValue) -> Self {
        Self {
            key: value.key,
            value: value.value,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct FileListModel {
    pub path: String,
}

impl From<File> for FileListModel {
    fn from(value: File) -> Self {
        Self {
            path: value.path,
        }
    }
}

// #[derive(Debug, Serialize)]
// pub(crate) struct ConfigListModel {
//     pub envvars: Vec::<KeyValue>,
//     pub files: Vec::<String>,
// }

// impl From<Config> for ConfigListModel {
//     fn from(value: Config) -> Self {
//         Self {
//             envvars: value.envvars,
//             files: value.files,
//         }
//     }
// }
