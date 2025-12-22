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

#[cfg(feature = "elephantry")]
impl Model {
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

    pub fn one(
        &self,
        token: &uuid::Uuid,
        item_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        let sql = include_str!("../sql/webhook.sql");
        self.connection
            .query::<Entity>(sql, &[item_id, token])
            .map(|x| x.try_get(0))
    }
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Response {
    #[serde(with = "serde_status")]
    pub status: reqwest::StatusCode,
    pub body: String,
}

mod serde_status {
    pub fn serialize<S>(value: &reqwest::StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(value.as_u16())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<reqwest::StatusCode, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize as _;

        let code = u16::deserialize(deserializer)?;

        Ok(reqwest::StatusCode::from_u16(code).unwrap())
    }
}
