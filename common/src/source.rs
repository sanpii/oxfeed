#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.source")
)]
pub struct Entity {
    pub last_error: Option<String>,
    #[cfg_attr(feature = "elephantry", elephantry(pk, column = "source_id"))]
    pub id: Option<uuid::Uuid>,
    pub tags: Vec<String>,
    pub title: String,
    pub url: String,
    pub user_id: uuid::Uuid,
    pub active: bool,
    #[cfg_attr(feature = "elephantry", elephantry(default))]
    pub webhooks: Vec<uuid::Uuid>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            last_error: None,
            id: None,
            tags: Vec::new(),
            title: String::new(),
            url: String::new(),
            user_id: uuid::Uuid::default(),
            active: true,
            webhooks: Vec::new(),
        }
    }
}

impl From<&Entity> for std::result::Result<std::string::String, anyhow::Error> {
    fn from(entity: &Entity) -> Self {
        let json = serde_json::to_string(entity)?;

        Ok(json)
    }
}

#[cfg(feature = "elephantry")]
impl<'a> Model<'a> {
    pub fn all(
        &self,
        token: &uuid::Uuid,
        filter: &elephantry::Where,
        pagination: &crate::Pagination,
    ) -> elephantry::Result<elephantry::Pager<Entity>> {
        let mut clause = filter.clone();
        clause.and_where("\"user\".token = $*", vec![token]);
        let params = clause.params();

        let query = format!(
            r#"
select source_id, last_error, title, url, user_id, active, webhooks,
        array(select unnest(tags) order by 1) tags
    from source
    join "user" using (user_id)
    where {}
    order by last_error is null, title
    {}
        "#,
            clause.to_string(),
            pagination.to_sql(),
        );

        let rows = self.connection.query::<Entity>(&query, &params)?;

        let query = format!(
            r#"
select count(*)
    from source
    join "user" using (user_id)
    where {}
        "#,
            clause.to_string()
        );

        let count = self.connection.query_one::<i64>(&query, &params)?;

        let pager = elephantry::Pager::new(rows, count as usize, pagination.page, pagination.limit);

        Ok(pager)
    }

    pub fn one(
        &self,
        token: &uuid::Uuid,
        source_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        let sql = include_str!("../sql/source.sql");
        self.connection
            .query::<Entity>(sql, &[source_id, token])
            .map(|x| x.try_get(0))
    }
}
