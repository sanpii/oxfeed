struct Model;

impl yew::Component for Model
{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self
    {
        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender
    {
        true
    }

    fn view(&self) -> yew::Html
    {
        yew::html! {
            <>
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender
    {
        false
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app()
{
    yew::initialize();
    yew::App::<Model>::new()
        .mount_to_body();
}
