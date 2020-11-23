#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: String,
}

pub(crate) struct Component(Properties);

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let bg_color = crate::cha::Color::from(&self.0.value);
        let color = if bg_color.is_dark() {
            "white"
        } else {
            "black"
        };
        let style = format!("background-color: {}; color: {}", bg_color.to_color_string(), color);

        yew::html! {
            <span style=style class="badge">{ &self.0.value }</span>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.0 != props;

        self.0 = props;

        should_render
    }
}
