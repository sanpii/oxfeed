#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub text: String,
}

pub(crate) struct Component {
    props: Properties,
}

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self {
            props,
        }
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
                    title="Last error"
                    text=&self.props.text
                    position="right"
                />
            </span>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.props.text != props.text;

        self.props.text = props.text;

        should_render
    }
}
