pub(crate) struct Component;

impl yew::Component for Component {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let export_url = format!("{}/opml", env!("API_URL"));

        yew::html! {
            <div class="card">
                <div class="card-header">{ "OPML" }</div>
                <div class="card-body">
                    <a href=export_url class=("btn", "btn-outline-primary")>{ "Export" }</a>
                </div>
            </div>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
