struct App;

impl yew::Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &yew::Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <oxfeed_front::components::App />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    yew::Renderer::<App>::new().render();
}
