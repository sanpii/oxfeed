impl<C> super::Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn user_create(&mut self, user: &crate::User) {
        self.fetch(super::Kind::UserCreate, http::Method::POST, "/users", user)
    }
}
