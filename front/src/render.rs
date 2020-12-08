pub(crate) trait Render: Clone + Eq + PartialEq {
    fn render(&self) -> yew::Html;
}

impl Render for String {
    fn render(&self) -> yew::Html {
        self.into()
    }
}

impl Render for oxfeed_common::item::Item {
    fn render(&self) -> yew::Html {
        yew::html! {
            <crate::components::Item value=self />
        }
    }
}

impl Render for oxfeed_common::source::Entity {
    fn render(&self) -> yew::Html {
        yew::html! {
            <crate::components::Source value=self />
        }
    }
}

impl Render for oxfeed_common::webhook::Entity {
    fn render(&self) -> yew::Html {
        yew::html! {
            <crate::components::Webhook value=self />
        }
    }
}
