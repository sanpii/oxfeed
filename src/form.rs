#[derive(serde::Deserialize)]
pub(crate) struct Source {
    source_id: Option<uuid::Uuid>,
    url: String,
    #[serde(default)]
    tags: Vec<String>,
}

impl Into<crate::model::source::Entity> for Source {
    fn into(self) -> crate::model::source::Entity {
        crate::model::source::Entity {
            url: self.url.clone(),
            source_id: self.source_id.clone(),
            title: String::new(),
            tags: self.tags.clone(),
        }
    }
}
