#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    yew::html! {
        <>
            <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ "Oxfeed" }}</a>
            <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <super::search::Bar />
            <super::Logout button=true />
        </>
    }
}
