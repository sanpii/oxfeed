#[derive(elephantry::Entity, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub email: String,
    pub password: String,
    pub name: String,
}

pub struct Model;

impl elephantry::Model<'_> for Model {
    type Entity = Entity;
    type Structure = super::user::Structure;

    fn new(_: &elephantry::Connection) -> Self {
        Self
    }
}
