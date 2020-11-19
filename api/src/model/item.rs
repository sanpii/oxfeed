#[derive(elephantry::Entity, serde::Serialize)]
pub struct Item {
    pub item_id: uuid::Uuid,
    pub link: String,
    pub published: chrono::DateTime<chrono::offset::Utc>,
    pub title: String,
    pub source: String,
    pub icon: Option<String>,
}

#[derive(elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub item_id: Option<String>,
    pub source_id: uuid::Uuid,
    pub id: String,
    pub link: String,
    pub title: String,
    pub content: Option<String>,
    pub read: bool,
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub icon: Option<String>,
}

pub struct Model<'a> {
    connection: &'a elephantry::Connection,
}

impl<'a> Model<'a> {
    pub fn unread(&self) -> elephantry::Result<elephantry::Rows<Item>> {
        let query = r#"
select item.item_id, item.link, item.published, item.title, item.icon,
        source.title as source
    from item
    join source using (source_id)
    where read = $*
    order by published desc
        "#;

        self.connection.query::<Item>(&query, &[&false])
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
            "published",
            "icon",
        ]
    }
}
