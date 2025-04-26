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
        let mut path = url.path_segments()?;

        path.next_back().map(ToString::to_string)
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.content_type != other.content_type {
            self.content_type.cmp(&other.content_type)
        } else {
            self.file_name().cmp(&other.file_name())
        }
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
