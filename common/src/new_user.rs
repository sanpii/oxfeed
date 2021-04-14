#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
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

#[cfg(feature = "elephantry")]
pub struct Model;

#[cfg(feature = "elephantry")]
impl elephantry::Model<'_> for Model {
    type Entity = Entity;
    type Structure = crate::user::Structure;

    fn new(_: &elephantry::Connection) -> Self {
        Self
    }
}
