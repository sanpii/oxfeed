#[derive(elephantry::Entity, serde::Serialize)]
pub struct Item {
    pub item_id: uuid::Uuid,
    pub link: String,
    pub published: chrono::DateTime<chrono::offset::Utc>,
    pub title: String,
    pub source: String,
    pub icon: Option<String>,
    pub read: bool,
    pub favorite: bool,
    pub tags: Vec<String>,
}

#[derive(elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub item_id: Option<uuid::Uuid>,
    pub source_id: uuid::Uuid,
    pub id: String,
    pub link: String,
    pub title: String,
    pub content: Option<String>,
    pub read: bool,
    pub favorite: bool,
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub icon: Option<String>,
}

pub struct Model<'a> {
    connection: &'a elephantry::Connection,
}

impl<'a> Model<'a> {
    pub fn all(
        &self,
        token: &uuid::Uuid,
        filter: &elephantry::Where,
        page: usize,
        max_per_page: usize
    ) -> elephantry::Result<elephantry::Pager<Item>> {
        let mut clause = filter.clone();
        clause.and_where("\"user\".token = $*", vec![token]);
        let params = clause.params();

        let query = format!(r#"
select item.item_id, item.link, item.published, item.title, item.icon,
        item.read, item.favorite, source.title as source, source.tags as tags
    from item
    join source using (source_id)
    join "user" using (user_id)
    where {}
    order by published desc
    offset {} fetch first {} rows only
        "#, clause.to_string(), (page - 1) * max_per_page, max_per_page);

        let rows = self.connection.query::<Item>(&query, &params)?;

        let query = format!(r#"
select count(*)
    from item
    join source using (source_id)
    join "user" using (user_id)
    where {}
        "#, clause.to_string());

        let count = self.connection.query_one::<i64>(&query, &params)?;

        let pager = elephantry::Pager::new(rows, count as usize, page, max_per_page);

        Ok(pager)
    }

    pub fn one(
        &self,
        token: &uuid::Uuid,
        item_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        self.connection.query::<Entity>(r#"
select item.*
    from item
    join source using(source_id)
    join "user" using(user_id)
    where item_id = $*
        and token = $*
        "#, &[item_id, token]).map(|x| x.try_get(0))
    }
}

impl<'a> elephantry::Model<'a> for Model<'a> {
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a elephantry::Connection) -> Self {
        Self {
            connection,
        }
    }
}

pub struct Structure;

impl elephantry::Structure for Structure {
    fn relation() -> &'static str {
        "public.item"
    }

    fn primary_key() -> &'static [&'static str] {
        &["item_id"]
    }

    fn columns() -> &'static [&'static str] {
        &[
            "item_id",
            "source_id",
            "id",
            "link",
            "title",
            "content",
            "read",
            "favorite",
            "published",
            "icon",
        ]
    }
}
