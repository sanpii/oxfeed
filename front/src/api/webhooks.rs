impl<C> super::Api<C>
where
    C: yew::Component,
    <C as yew::Component>::Message: From<crate::event::Api>,
{
    pub fn webhooks_all(&mut self) {
        self.fetch(
            super::Kind::Webhooks,
            http::Method::GET,
            "/webhooks",
            yew::format::Nothing,
        )
    }

    pub fn webhooks_create(&mut self, webhook: &oxfeed_common::webhook::Entity) {
        self.fetch(
            super::Kind::WebhookCreate,
            http::Method::POST,
            "/webhooks",
            webhook,
        )
    }

    pub fn webhooks_update(&mut self, id: &uuid::Uuid, webhook: &oxfeed_common::webhook::Entity) {
        self.fetch(
            super::Kind::WebhookUpdate,
            http::Method::PUT,
            &format!("/webhooks/{}", id),
            webhook,
        )
    }

    pub fn webhooks_delete(&mut self, id: &uuid::Uuid) {
        self.fetch(
            super::Kind::WebhookDelete,
            http::Method::DELETE,
            &format!("/webhooks/{}", id),
            yew::format::Nothing,
        )
    }
}
