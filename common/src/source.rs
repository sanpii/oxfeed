#[derive(Clone, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Entity {
    pub last_error: Option<String>,
    pub source_id: Option<uuid::Uuid>,
    pub tags: Vec<String>,
    pub title: String,
    pub url: String,
    pub user_id: uuid::Uuid,
    pub active: bool,
    #[cfg_attr(feature = "elephantry", elephantry(default))]
    pub webhooks: Vec<uuid::Uuid>,
}

impl Into<std::result::Result<std::string::String, anyhow::Error>> for &Entity {
    fn into(self) -> std::result::Result<std::string::String, anyhow::Error> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

#[cfg(feature = "elephantry")]
pub struct Model<'a> {
    connection: &'a elephantry::Connection,
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
select *
    from source
    join "user" using (user_id)
    where {}
    order by last_error, title
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
        "public.source"
    }

    fn primary_key() -> &'static [&'static str] {
        &["source_id"]
    }

    fn columns() -> &'static [&'static str] {
        &["source_id", "user_id", "title", "tags", "url", "last_error", "active", "webhooks"]
    }
}
