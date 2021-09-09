use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;

pub type ID = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Structure {
    pub id: ID,
    pub name: String,
}

