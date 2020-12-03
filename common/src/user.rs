#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Entity {
    pub user_id: uuid::Uuid,
    pub email: String,
}

#[cfg(feature = "elephantry")]
pub struct Model<'a> {
    connection: &'a elephantry::Connection,
}

#[cfg(feature = "elephantry")]
impl<'a> Model<'a> {
    pub fn find_from_token(&self, token: &uuid::Uuid) -> Option<Entity> {
        self.connection
            .find_where::<Self>("token = $*", &[token], None)
            .map(|x| x.get(0))
            .ok()
    }
}

#[cfg(feature = "elephantry")]
impl<'a> elephantry::Model<'a> for Model<'a> {
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a elephantry::Connection) -> Self {
        Self { connection }
    }

    fn create_projection() -> elephantry::Projection {
        Self::default_projection().unset_field("password")
    }
}

#[cfg(feature = "elephantry")]
pub struct Structure;

#[cfg(feature = "elephantry")]
impl elephantry::Structure for Structure {
    fn relation() -> &'static str {
        "public.user"
    }

    fn primary_key() -> &'static [&'static str] {
        &["user_id"]
    }

    fn columns() -> &'static [&'static str] {
        &["user_id", "email", "password"]
    }
}
