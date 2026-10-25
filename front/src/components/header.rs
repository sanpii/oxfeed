#[yew::component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let fetching = yew::use_memo(context, |context| context.fetching);

    let title = if *fetching {
        yew::html! { <super::Svg icon="arrow-clockwise" size=24 /> }
    } else {
        yew::html! { "Ofxeed" }
    };

    yew::html! {
        <nav class="d-flex navbar lg:navbar-expand sticky-top md:flex-nowrap p-0 shadow" data-bs-theme="dark">
            <a class="navbar-brand md:col-3 lg:col-2 ms-3 me-0 px-3" href="#">{{ title }}</a>
            <super::search::Bar />
            <button class="btn-icon navbar-toggler mx-2" type="button" data-bs-toggle="drawer" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <super::Theme button=true />
            <super::Logout button=true />
        </nav>
    }
}
