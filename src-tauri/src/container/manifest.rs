use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub format_version: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub default_language: String,
    pub entry_scene: String,
    pub catalogs: Catalogs,
    pub capabilities: Capabilities,
}

#[derive(Debug, Deserialize)]
pub struct Catalogs {
    pub scenes: String,
    pub resources: String,
    pub state: String,
    pub profile: Option<String>,
    pub characters: Option<String>,
    pub settings: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Capabilities {
    pub required: Vec<String>,
    pub optional: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SceneCatalogEntry {
    pub href: String,
    pub mode: Option<String>,
    pub chapter: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SceneCatalog {
    pub scenes: HashMap<String, SceneCatalogEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceEntry {
    pub href: String,
    pub media_type: String,
    pub role: String,
    pub description: Option<String>,
    pub fallback: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceCatalog {
    pub resources: HashMap<String, ResourceEntry>,
}
