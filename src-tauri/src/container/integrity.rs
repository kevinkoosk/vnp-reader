use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IntegrityEntry {
    pub path: String,
    pub size: u64,
    pub digest: String,
}

#[derive(Debug, Deserialize)]
pub struct IntegrityDocument {
    pub algorithm: String,
    pub entries: Vec<IntegrityEntry>,
}
