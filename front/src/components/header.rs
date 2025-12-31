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
        <nav class="d-flex navbar navbar-dark navbar-expand-lg sticky-top bg-dark flex-md-nowrap p-0 shadow">
            <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ title }}</a>
            <super::search::Bar />
            <button class="navbar-toggler collapsed mx-2" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <super::Theme button=true />
            <super::Logout button=true />
        </nav>
    }
}
