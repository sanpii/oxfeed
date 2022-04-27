#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.user")
)]
pub struct Entity {
    #[cfg_attr(feature = "elephantry", elephantry(pk, column = "user_id"))]
    pub id: Option<uuid::Uuid>,
    pub email: String,
    pub password: String,
}

impl From<&Entity> for crate::Result<String> {
    fn from(entity: &Entity) -> Self {
        let json = serde_json::to_string(entity)?;

        Ok(json)
    }
}

impl From<crate::user::Entity> for Entity {
    fn from(user: crate::user::Entity) -> Self {
        Self {
            id: Some(user.id),
            email: user.email,
            password: String::new(),
        }
    }
}

#[cfg(feature = "elephantry")]
impl Model {
    pub fn find_from_token(&self, token: &uuid::Uuid) -> Option<Entity> {
        self.connection
            .find_where::<Self>("token = $*", &[token], None)
            .map(|x| x.get(0))
            .ok()
    }
}
