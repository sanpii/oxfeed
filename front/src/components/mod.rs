mod actions;
mod header;
mod form;
mod item;
mod items;
mod sidebar;
mod source;
mod sources;
mod svg;

pub(crate) use actions::Component as Actions;
pub(crate) use form::Component as Form;
pub(crate) use header::Component as Header;
pub(crate) use item::Component as Item;
pub(crate) use items::Component as Items;
pub(crate) use sidebar::Component as Sidebar;
pub(crate) use source::Component as Source;
pub(crate) use sources::Component as Sources;
pub(crate) use svg::Component as Svg;

macro_rules! decl_items {
    ($name:ident) => {
        pub(crate) struct $name;

        impl yew::Component for $name {
            type Message = ();
            type Properties = ();

            fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
                Self
            }

            fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
                false
            }

            fn view(&self) -> yew::Html {
                yew::html! {
                    <super::Items filter=stringify!($name).to_lowercase() />
                }
            }

            fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
                false
            }
        }
    }
}

decl_items!(All);
decl_items!(Favorites);
decl_items!(Unread);
