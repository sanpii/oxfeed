#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let fetching = yew::use_memo(context, |context| context.fetching);

    let title = if *fetching {
        yew::html! { <super::Svg icon="arrow-clockwise" size=24 /> }
    } else {
        yew::html! { "Ofxeed" }
    };

    yew::html! {
        <>
            <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ title }}</a>
            <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <super::search::Bar />
            <ul class="navbar-nav ms-auto">
                <li class="nav-item"><super::Theme /></li>
                <li class="nav-item"><super::Logout button=true /></li>
            </ul>
        </>
    }
}
