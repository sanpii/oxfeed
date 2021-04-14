#[derive(Clone, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Entity {
    pub webhook_id: Option<uuid::Uuid>,
    pub user_id: Option<uuid::Uuid>,
    pub name: String,
    pub url: String,
    pub last_error: Option<String>,
    pub mark_read: bool,
}

impl From<&Entity> for std::result::Result<std::string::String, anyhow::Error> {
    fn from(entity: &Entity) -> Self {
        let json = serde_json::to_string(entity)?;

        Ok(json)
    }
}

#[cfg(feature = "elephantry")]
pub struct Model<'a> {
    connection: &'a elephantry::Connection,
}

#[cfg(feature = "elephantry")]
impl<'a> Model<'a> {
    pub fn delete(
        &self,
        token: &uuid::Uuid,
        webhook_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        let sql = include_str!("../sql/webhook-delete.sql");
        self.connection
            .query::<Entity>(sql, &[webhook_id, token])
            .map(|x| x.try_get(0))
    }
}

#[cfg(feature = "elephantry")]
impl<'a> Model<'a> {
    pub fn all(&self, token: &uuid::Uuid) -> elephantry::Result<elephantry::Rows<Entity>> {
        let sql = include_str!("../sql/webhooks.sql");
        self.connection.query::<Entity>(sql, &[token])
    }
}

#[cfg(feature = "elephantry")]
impl<'a> elephantry::Model<'a> for Model<'a> {
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a elephantry::Connection) -> Self {
        Self { connection }
    }
}

#[cfg(feature = "elephantry")]
pub struct Structure;

#[cfg(feature = "elephantry")]
impl elephantry::Structure for Structure {
    fn relation() -> &'static str {
        "public.webhook"
    }

    fn primary_key() -> &'static [&'static str] {
        &["webhook_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "webhook_id",
            "user_id",
            "name",
            "url",
            "last_error",
            "mark_read",
        ]
    }
}
