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

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <components::App />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::initialize();
    yew::App::<App>::new().mount_to_body();
}

#[macro_export]
macro_rules! change {
    () => {
        fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
            false
        }
    };

    (props) => {
        fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
            let should_render = self.props != props;

            self.props = props;

            should_render
        }
    };

    ($(props.$prop: ident),+) => {
        fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
            let should_render = false
                $(
                    || self.props.$prop != props.$prop
                )*;

            self.props = props;

            should_render
        }
    };

    ($($prop: ident),+) => {
        fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
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
    ($link:expr, $api:ident ( $($args:ident),* ) -> $fn:expr) => {
        $crate::api!($link, $api ( $($args),* ) -> $fn, |err| Message::Event(err.into()))
    };

    ($link:expr, $api:ident ( $($args:ident),* ) -> $ok:expr, $err:expr) => {{
        use yewtil::future::LinkFuture;

        $( let $args = $args.clone(); )*

        $link.send_future(async move {
            crate::Api::$api($( &$args ),*).await.map_or_else($err, $ok)
        });
    }};
}
