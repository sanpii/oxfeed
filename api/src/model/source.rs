#[derive(Debug, elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub last_error: Option<String>,
    pub source_id: Option<uuid::Uuid>,
    pub tags: Vec<String>,
    pub title: String,
    pub url: String,
    pub user_id: uuid::Uuid,
}

impl std::convert::TryFrom<(&opml::Outline, &uuid::Uuid)> for Entity {
    type Error = ();

    fn try_from((outline, user_id): (&opml::Outline, &uuid::Uuid)) -> Result<Self, Self::Error> {
        let url = match &outline.xml_url {
            Some(url) => url.clone(),
            None => return Err(()),
        };

        let mut tags = Vec::new();

        if let Some(category) = &outline.category {
            tags.push(category.clone());
        }

        let entity = Self {
            last_error: None,
            source_id: None,
            tags,
            title: outline.text.clone(),
            url,
            user_id: user_id.clone(),
        };

        Ok(entity)
    }
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
        max_per_page: usize,
    ) -> elephantry::Result<elephantry::Pager<Entity>> {
        let mut clause = filter.clone();
        clause.and_where("\"user\".token = $*", vec![token]);
        let params = clause.params();

        let query = format!(r#"
select *
    from source
    join "user" using (user_id)
    where {}
    order by last_error, title
    offset {} fetch first {} rows only
        "#, clause.to_string(), (page - 1) * max_per_page, max_per_page);

        let rows = self.connection.query::<Entity>(&query, &params)?;

        let query = format!(r#"
select count(*)
    from source
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
        source_id: &uuid::Uuid,
    ) -> elephantry::Result<Option<Entity>> {
        let sql = include_str!("../sql/source.sql");
        self.connection.query::<Entity>(sql, &[source_id, token]).map(|x| x.try_get(0))
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
        "public.source"
    }

    fn primary_key() -> &'static [&'static str] {
        &["source_id"]
    }

    fn columns() -> &'static [&'static str] {
        &["source_id", "user_id", "title", "tags", "url", "last_error"]
    }
}
