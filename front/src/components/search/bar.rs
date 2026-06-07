#[yew::component]
pub(crate) fn Component() -> yew::Html {
    let location = crate::use_location();
    let route = yew_router::hooks::use_route::<crate::components::app::Route>().unwrap_or_default();
    let filter = yew::use_memo(location.clone(), |x| crate::Filter::new(x));

    let on_input = yew_callback::callback!(location, move |e: yew::InputEvent| {
        use yew::TargetCast as _;
        use yew_router::history::History as _;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        let filter: crate::Filter = input.value().into();

        let mut route = location.path();

        if route.starts_with("/search") {
            route = route.trim_start_matches("/search").to_string();
        }

        if !filter.is_empty() {
            route = format!("/search{route}?{}", filter.to_url_param());
        }

        let history = yew_router::history::BrowserHistory::new();
        history.push(&route);
    });

    if matches!(route, crate::components::app::Route::Settings) {
        yew::Html::default()
    } else {
        yew::html! {
            <input
                class="ms-2 form-control form-control-dark flex-grow-1"
                type="text"
                value={ filter.to_string() }
                placeholder="Search"
                aria-label="Search"
                oninput={ on_input }
            />
        }
    }
}
