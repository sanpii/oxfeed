#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    yew::html! {
        <div class="full-page">
            <super::Svg icon="file-earmark-x" size=256 />
            <h2>{ "Page not found" }</h2>
        </div>
    }
}
