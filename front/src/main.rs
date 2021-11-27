#![warn(warnings)]
#![recursion_limit = "1024"]

mod api;
mod cha;
mod components;
mod event;
mod filter;
mod location;
mod render;

pub(crate) use api::Api;
pub(crate) use event::Event;
pub(crate) use filter::*;
pub(crate) use location::Location;
pub(crate) use render::*;

#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
struct Pager<R: Render> {
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
    iterator: Vec<R>,
}

impl<R: crate::Render> From<Pager<R>> for elephantry_extras::Pager {
    fn from(pager: Pager<R>) -> Self {
        elephantry_extras::Pager {
            count: pager.count,
            page: pager.page,
            max_per_page: pager.max_per_page,
        }
    }
}

struct App;

impl yew::Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &yew::Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <components::App />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    yew::start_app::<App>();
}

#[macro_export]
macro_rules! change {
    () => {
        fn changed(&mut self, _: &yew::Context<Self>) -> bool {
            false
        }
    };

    (props) => {
        fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
            let should_render = &self.props != ctx.props();

            self.props = ctx.props().clone();

            should_render
        }
    };

    ($(props.$prop: ident),+) => {
        fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
            let should_render = false
                $(
                    || self.props.$prop != ctx.props().$prop
                )*;

            self.props = ctx.props().clone();

            should_render
        }
    };

    ($($prop: ident),+) => {
        fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
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
            crate::Api::$api($( &$args ),*).await.map_or_else(
                |err| Message::Error(err.to_string()),
                $fn
            )
        });
    }}
}
