#[derive(yew::Properties, Clone)]
pub(crate) struct Properties {
    pub value: crate::Source,
}

pub(crate) struct Component(crate::Source);

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self(props.value)
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div>{ &self.0.title }</div>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
