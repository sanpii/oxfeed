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
        let contents = [
            ("book", "You really want to read something? Take a book!"),
            ("cup", "Nothing here, it’s time to boil water while waiting something to read arrive!"),
            ("pencil-square", "Nothing to read? Write what you want to read!"),
        ];

        let now = chrono::Utc::now();
        let choice = now.timestamp_subsec_nanos() as usize % contents.len();

        yew::html! {
            <div class="empty">
                <super::Svg icon=contents[choice].0 size=256 />
                <h2>{ contents[choice].1 }</h2>
            </div>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
