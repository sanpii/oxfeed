#[derive(Clone, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.webhook")
)]
pub struct Entity {
    #[cfg_attr(feature = "elephantry", elephantry(pk, column = "webhook_id"))]
    pub id: Option<uuid::Uuid>,
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

    pub fn all(&self, token: &uuid::Uuid) -> elephantry::Result<elephantry::Rows<Entity>> {
        let sql = include_str!("../sql/webhooks.sql");
        self.connection.query::<Entity>(sql, &[token])
    }
}
