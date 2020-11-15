#[derive(elephantry::Entity)]
pub struct Entity {
    pub entry_id: String,
    pub source_id: String,
    pub link: String,
    pub title: String,
    pub content: String,
    pub icon: Option<String>,
    pub read_at: Option<String>,
    pub created_at: String,
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
            "icon",
            "read_at",
            "created_at",
        ]
    }
}
