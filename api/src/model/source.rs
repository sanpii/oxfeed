#[derive(Debug, elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub source_id: Option<uuid::Uuid>,
    pub title: String,
    pub tags: Vec<String>,
    pub url: String,
}

pub struct Model;

impl elephantry::Model<'_> for Model {
    type Entity = Entity;
    type Structure = Structure;

    fn new(_: &elephantry::Connection) -> Self {
        Self {}
    }
}

pub struct Structure;

impl elephantry::Structure for Structure {
    fn relation() -> &'static str {
        "public.source"
    }

    fn primary_key() -> &'static [&'static str] {
        &["source_id"]
    }

    fn columns() -> &'static [&'static str] {
        &["source_id", "title", "tags", "url"]
    }
}
