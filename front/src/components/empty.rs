pub struct Component;

impl yew::Component for Component {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        let contents = [
            ("book", "You really want to read something? Take a book!"),
            (
                "cup",
                "Nothing here, it’s time to boil water while waiting something to read arrive!",
            ),
            (
                "pencil-square",
                "Nothing to read? Write what you want to read!",
            ),
        ];

        let now = web_sys::window().unwrap().performance().unwrap().now();
        let choice = now as usize % contents.len();

        yew::html! {
            <div class="full-page">
                <super::Svg icon={ contents[choice].0 } size=256 />
                <h2>{ contents[choice].1 }</h2>
            </div>
        }
    }

    crate::change!();
}
