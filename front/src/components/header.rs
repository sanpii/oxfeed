#[derive(PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: super::app::Route,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let filter = yew::use_memo(context.route.clone(), |_| crate::Filter::new());

    yew::html! {
        <>
            <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ "Oxfeed" }}</a>
            <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <super::search::Bar current_route={ props.current_route.clone() } filter={ (*filter).clone() } />
            <super::Logout button=true />
        </>
    }
}
