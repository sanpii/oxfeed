#![warn(warnings)]
#![recursion_limit = "1024"]

mod api;
mod cha;
mod filter;
mod location;
mod render;

pub mod components;
pub mod event;

pub use api::Api;
pub use event::Event;
pub use filter::*;
pub use location::Location;
pub use render::*;

#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
pub struct Pager<T> {
    result_count: usize,
    result_min: usize,
    result_max: usize,
    last_page: usize,
    page: usize,
    has_next_page: bool,
    has_previous_page: bool,
    count: usize,
    max_per_page: usize,
    #[serde(default = "location::base_url")]
    base_url: String,
    iterator: Vec<T>,
}

impl<T: crate::Render> From<Pager<T>> for elephantry_extras::Pager {
    fn from(pager: Pager<T>) -> Self {
        elephantry_extras::Pager {
            count: pager.count,
            page: pager.page,
            max_per_page: pager.max_per_page,
        }
    }
}

#[macro_export]
macro_rules! change {
    () => {
        fn changed(&mut self, _: &yew::Context<Self>, _: &Self::Properties) -> bool {
            false
        }
    };

    (props) => {
        fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
            let should_render = &self.props != ctx.props();

            self.props = ctx.props().clone();

            should_render
        }
    };

    ($(props.$prop: ident),+) => {
        fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
            let should_render = false
                $(
                    || self.props.$prop != ctx.props().$prop
                )*;

            self.props = ctx.props().clone();

            should_render
        }
    };

    ($($prop: ident),+) => {
        fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
            let props = ctx.props().clone();

            let should_render = false
                $(
                    || self.$prop != props.$prop
                )*;

            $(
                self.$prop = props.$prop;
            )*

            should_render
        }
    };
}

#[macro_export]
macro_rules! api {
    ($link:expr, $api:ident ( $($args:ident),* ) -> $fn:expr) => {{
        $( let $args = $args.clone(); )*

        $link.send_future(async move {
            $crate::Api::$api($( &$args ),*).await.map_or_else(
                |err| Message::Error(err.to_string()),
                $fn
            )
        });
    }}
}
