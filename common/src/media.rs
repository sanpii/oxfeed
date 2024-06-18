#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(
    feature = "elephantry",
    derive(elephantry::Entity, elephantry::Composite)
)]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.media")
)]
pub struct Entity {
    #[cfg_attr(feature = "elephantry", elephantry(column = "media_id"))]
    pub id: Option<uuid::Uuid>,
    pub item_id: uuid::Uuid,
    pub url: String,
    pub content_type: Option<String>,
}

impl Entity {
    pub fn file_name(&self) -> Option<String> {
        let url = url::Url::parse(&self.url).ok()?;
        let path = url.path_segments()?;

        path.last().map(ToString::to_string)
    }
}
