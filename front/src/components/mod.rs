mod actions;
mod alerts;
mod app;
mod header;
mod form;
mod item;
mod items;
mod pager;
mod search;
mod settings;
mod sidebar;
mod source;
mod sources;
mod svg;
mod tag;

pub(crate) use actions::Component as Actions;
pub(crate) use alerts::Component as Alerts;
pub(crate) use app::Component as App;
pub(crate) use form::Component as Form;
pub(crate) use header::Component as Header;
pub(crate) use item::Component as Item;
pub(crate) use items::Component as Items;
pub(crate) use pager::Component as Pager;
pub(crate) use search::Component as Search;
pub(crate) use settings::Component as Settings;
pub(crate) use sidebar::Component as Sidebar;
pub(crate) use source::Component as Source;
pub(crate) use sources::Component as Sources;
pub(crate) use svg::Component as Svg;
pub(crate) use tag::Component as Tag;

macro_rules! decl_items {
    ($name:ident) => {
        mod $name {
            #[derive(Clone, yew::Properties)]
            pub(crate) struct Properties {
                pub pagination: $crate::Pagination,
            }

            pub(crate) struct Component {
                pagination: $crate::Pagination,
            }

            impl yew::Component for Component {
                type Message = ();
                type Properties = Properties;

                fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
                    Self {
                        pagination: props.pagination,
                    }
                }

                fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
                    false
                }

                fn view(&self) -> yew::Html {
                    let name = stringify!($name);
                    let filter = if name == "all" {
                        "/items".to_string()
                    } else {
                        format!("/items/{}", name)
                    };

                    yew::html! {
                        <super::Items filter=filter pagination=self.pagination />
                    }
                }

                fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
                    let should_render = self.pagination != props.pagination;

                    self.pagination = props.pagination;

                    should_render
                }
            }
        }

    }
}

decl_items!(all);
decl_items!(favorites);
decl_items!(unread);

pub(crate) use all::Component as All;
pub(crate) use favorites::Component as Favorites;
pub(crate) use unread::Component as Unread;
