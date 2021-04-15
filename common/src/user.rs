#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "elephantry", derive(elephantry::Entity))]
#[cfg_attr(
    feature = "elephantry",
    elephantry(model = "Model", structure = "Structure", relation = "public.user")
)]
pub struct Entity {
    #[cfg_attr(feature = "elephantry", elephantry(pk, column = "user_id"))]
    pub id: uuid::Uuid,
    pub email: String,
}

#[cfg(feature = "elephantry")]
impl<'a> Model<'a> {
    pub fn find_from_token(&self, token: &uuid::Uuid) -> Option<Entity> {
        self.connection
            .find_where::<Self>("token = $*", &[token], None)
            .map(|x| x.get(0))
            .ok()
    }
}
