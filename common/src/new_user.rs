#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.user")
)]
pub struct Entity {
    pub email: String,
    pub password: String,
}

impl From<&Entity> for std::result::Result<std::string::String, anyhow::Error> {
    fn from(entity: &Entity) -> Self {
        let json = serde_json::to_string(entity)?;

        Ok(json)
    }
}
