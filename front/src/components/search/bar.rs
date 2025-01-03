#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: crate::components::app::Route,
    #[prop_or_default]
    pub filter: crate::Filter,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let current_route = yew::use_memo(props.clone(), |props| props.current_route.clone());
    let filter = yew::use_state(crate::Filter::new);

    let on_input = {
        let filter = filter.clone();

        yew::Callback::from(move |e: yew::InputEvent| {
            use yew::TargetCast as _;

            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            filter.set(input.value().into());
        })
    };

    let on_search = {
        let filter = filter.clone();

        yew::Callback::from(move |_| {
            let location = crate::Location::new();
            let mut route = location.path();

            if route.starts_with("/search") {
                route = route.trim_start_matches("/search").to_string();
            }

            if !filter.is_empty() {
                route = format!("/search{route}?{}", filter.to_url_param());
            }

            context.dispatch(crate::Action::Route(route));
        })
    };

    if matches!(*current_route, crate::components::app::Route::Settings) {
        "".into()
    } else {
        yew::html! {
            <form method="get">
                <div class="input-group">
                    <input
                        class={ yew::classes!("form-control", "form-control-dark") }
                        type="text"
                        name="q"
                        value={ filter.to_string() }
                        placeholder="Search"
                        aria-label="Search"
                        oninput={ on_input }
                    />
                    <button class="btn btn-outline-secondary" type="button" onclick={ on_search }>
                        <crate::components::Svg icon="search" size=24 />
                    </button>
                </div>
            </form>
        }
    }
}
