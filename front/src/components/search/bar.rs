#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let route = yew_router::hooks::use_route::<crate::components::app::Route>().unwrap_or_default();
    let location = yew_router::prelude::use_location();
    let filter = yew::use_memo(location.clone(), |_| crate::Filter::new());

    let on_input = yew_callback::callback!(move |e: yew::InputEvent| {
        use yew::TargetCast as _;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        let filter: crate::Filter = input.value().into();

        let location = crate::Location::new();
        let mut route = location.path();

        if route.starts_with("/search") {
            route = route.trim_start_matches("/search").to_string();
        }

        if !filter.is_empty() {
            route = format!("/search{route}?{}", filter.to_url_param());
        }

        context.dispatch(crate::Action::Route(route));
    });

    if matches!(route, crate::components::app::Route::Settings) {
        yew::Html::default()
    } else {
        yew::html! {
            <input
                class="form-control form-control-dark"
                type="text"
                value={ filter.to_string() }
                placeholder="Search"
                aria-label="Search"
                oninput={ on_input }
            />
        }
    }
}
