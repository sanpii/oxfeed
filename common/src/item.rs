#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
pub struct Item {
    #[cfg_attr(feature = "elephantry", elephantry(column = "item_id"))]
    pub id: uuid::Uuid,
    pub link: String,
    pub published: chrono::DateTime<chrono::offset::Utc>,
    pub title: String,
    pub source: String,
    pub icon: Option<String>,
    pub read: bool,
    pub favorite: bool,
    pub tags: Vec<String>,
}

#[derive(serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.item")
)]
pub struct Entity {
    #[cfg_attr(feature = "elephantry", elephantry(pk, column = "item_id"))]
    pub id: Option<uuid::Uuid>,
    pub source_id: uuid::Uuid,
    #[cfg_attr(feature = "elephantry", elephantry(column = "id"))]
    pub feed_id: String,
    pub link: String,
    pub title: String,
    pub content: Option<String>,
    pub read: bool,
    pub favorite: bool,
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub icon: Option<String>,
}

#[cfg(feature = "elephantry")]
impl Model {
    pub fn all(
        &self,
        token: &uuid::Uuid,
        filter: &elephantry::Where,
        pagination: &crate::Pagination,
    ) -> elephantry::Result<elephantry::Pager<Item>> {
        let mut clause = filter.clone();
        clause.and_where("\"user\".token = $*", vec![token]);
        let params = clause.params();

        let query = format!(
            r#"
select item.item_id, item.link, item.published, item.title,
        '/icons/' || encode(convert_to(item.icon, 'utf8'), 'base64') as icon,
        item.read, item.favorite, source.title as source, source.tags as tags
    from item
    join source using (source_id)
    join "user" using (user_id)
    where {}
    order by published desc, title
    {}
        "#,
            clause.to_string(),
            pagination.to_sql(),
        );

        let rows = self.connection.query::<Item>(&query, &params)?;

        let query = format!(
            r#"
select count(*)
    from item
    join source using (source_id)
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
        item_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        let sql = include_str!("../sql/item.sql");
        self.connection
            .query::<Entity>(sql, &[item_id, token])
            .map(|x| x.try_get(0))
    }
}
