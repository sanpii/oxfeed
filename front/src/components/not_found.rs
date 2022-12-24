pub struct Component;

impl yew::Component for Component {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class="full-page">
                <super::Svg icon="file-earmark-x" size=256 />
                <h2>{ "Page not found" }</h2>
            </div>
        }
    }

    crate::change!();
}
