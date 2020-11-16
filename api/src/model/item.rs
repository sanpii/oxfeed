#[derive(elephantry::Entity)]
pub struct Entity {
    pub entry_id: Option<String>,
    pub source_id: uuid::Uuid,
    pub link: String,
    pub title: String,
    pub content: Option<String>,
    pub read: bool,
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
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
        "public.item"
    }

    fn primary_key() -> &'static [&'static str] {
        &["entry_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "entry_id",
            "source_id",
            "link",
            "title",
            "content",
            "read",
            "published",
        ]
    }
}
