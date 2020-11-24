#[derive(Debug, elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub last_error: Option<String>,
    pub source_id: Option<uuid::Uuid>,
    pub tags: Vec<String>,
    pub title: String,
    pub url: String,
}

impl std::convert::TryFrom<&opml::Outline> for Entity {
    type Error = ();

    fn try_from(outline: &opml::Outline) -> Result<Self, Self::Error> {
        let url = match &outline.xml_url {
            Some(url) => url.clone(),
            None => return Err(()),
        };

        let mut tags = Vec::new();

        if let Some(category) = &outline.category {
            tags.push(category.clone());
        }

        let entity = Self {
            last_error: None,
            source_id: None,
            tags,
            title: outline.text.clone(),
            url,
        };

        Ok(entity)
    }
}

pub struct Model;

impl elephantry::Model<'_> for Model {
    type Entity = Entity;
    type Structure = Structure;

    fn new(_: &elephantry::Connection) -> Self {
        Self
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
        &["source_id", "title", "tags", "url", "last_error"]
    }
}
