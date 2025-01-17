pub(crate) mod app;

mod actions;
mod alerts;
mod empty;
mod error;
mod form;
mod header;
mod item;
mod items;
mod login;
mod logout;
mod media;
mod not_found;
mod popover;
mod search;
mod settings;
mod sidebar;
mod source;
mod sources;
mod svg;
mod swipe;
mod switch;
mod tag;
mod tags;
mod webhook;

pub(crate) use actions::Component as Actions;
pub(crate) use alerts::Component as Alerts;
pub(crate) use app::Component as App;
pub(crate) use empty::Component as Empty;
pub(crate) use error::Component as Error;
pub(crate) use header::Component as Header;
pub(crate) use item::Component as Item;
pub(crate) use items::Component as Items;
pub(crate) use login::Component as Login;
pub(crate) use logout::Component as Logout;
pub(crate) use media::Component as Media;
pub(crate) use not_found::Component as NotFound;
pub(crate) use popover::Component as Popover;
pub(crate) use search::Component as Search;
pub(crate) use settings::Component as Settings;
pub(crate) use sidebar::Component as Sidebar;
pub(crate) use source::Component as Source;
pub(crate) use sources::Component as Sources;
pub(crate) use svg::Component as Svg;
pub(crate) use swipe::Component as Swipe;
pub(crate) use switch::Component as Switch;
pub(crate) use tag::Component as Tag;
pub(crate) use tags::Component as Tags;
pub(crate) use webhook::Component as Webhook;

pub(crate) fn edit_cb(
    state: yew::functional::UseStateHandle<String>,
) -> yew::Callback<yew::InputEvent> {
    yew::Callback::from(move |e: yew::InputEvent| {
        use yew::TargetCast as _;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        input.report_validity();

        state.set(input.value());
    })
}

pub(crate) fn toggle_cb(
    state: yew::functional::UseStateHandle<bool>,
) -> yew::Callback<yew::MouseEvent> {
    yew::Callback::from(move |_| {
        state.set(!(*state));
    })
}
