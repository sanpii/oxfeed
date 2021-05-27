#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub text: String,
}

pub(crate) struct Component {
    text: String,
}

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self { text: props.text }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <span class="help">
                {" "}
                <crate::components::Svg icon="exclamation-octagon" size=16 class="text-danger" />
                <crate::components::Popover
                    title="Last error".to_string()
                    text=self.text.clone()
                    position="end"
                />
            </span>
        }
    }

    crate::change!(text);
}
