#[derive(elephantry::Entity, serde::Serialize)]
pub struct Entity {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub name: String,
}

pub struct Model<'a> {
    connection: &'a elephantry::Connection,
}

impl<'a> Model<'a> {
    pub fn find_from_identity(&self, identity: &crate::Identity) -> Option<Entity> {
        let token = match identity.token() {
            Some(token) => token,
            None => return None,
        };

        self.find_from_token(&token)
    }

    fn find_from_token(&self, token: &uuid::Uuid) -> Option<Entity> {
        self.connection
            .find_where::<Self>("token = $*", &[token], None)
            .map(|x| x.get(0))
            .ok()
    }
}

impl<'a> elephantry::Model<'a> for Model<'a> {
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a elephantry::Connection) -> Self {
        Self { connection }
    }

    fn create_projection() -> elephantry::Projection {
        Self::default_projection().unset_field("password")
    }
}

pub struct Structure;

impl elephantry::Structure for Structure {
    fn relation() -> &'static str {
        "public.user"
    }

    fn primary_key() -> &'static [&'static str] {
        &["user_id"]
    }

    fn columns() -> &'static [&'static str] {
        &["user_id", "name", "email", "password"]
    }
}
