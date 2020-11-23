#[derive(Debug, elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub source_id: Option<uuid::Uuid>,
    pub title: String,
    pub tags: Vec<String>,
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
            source_id: None,
            title: outline.text.clone(),
            tags,
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
