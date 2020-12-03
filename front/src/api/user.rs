impl<C> super::Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn user_create(&mut self, user: &oxfeed_common::new_user::Entity) {
        self.fetch(super::Kind::UserCreate, http::Method::POST, "/users", user)
    }
}
