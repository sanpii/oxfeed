#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.filter")
)]
pub struct Entity {
    #[cfg_attr(feature = "elephantry", elephantry(pk, column = "filter_id"))]
    pub id: Option<uuid::Uuid>,
    pub user_id: Option<uuid::Uuid>,
    pub name: String,
    pub regex: String,
}

#[cfg(feature = "elephantry")]
impl Model {
    pub fn delete(
        &self,
        token: &uuid::Uuid,
        filter_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        let sql = include_str!("../sql/filter-delete.sql");
        self.connection
            .query::<Entity>(sql, &[filter_id, token])
            .map(|x| x.try_get(0))
    }

    pub fn all(&self, token: &uuid::Uuid) -> elephantry::Result<elephantry::Rows<Entity>> {
        let sql = include_str!("../sql/filters.sql");
        self.connection.query::<Entity>(sql, &[token])
    }
}
