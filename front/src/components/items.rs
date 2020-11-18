#[derive(Clone)]
pub(crate) enum Message {
    Nothing,
}

impl std::convert::TryFrom<yew::format::Text> for Message {
    type Error = ();

    fn try_from(_: yew::format::Text) -> Result<Self, ()> {
        Ok(Message::Nothing)
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub value: Vec<crate::Item>,
}

pub(crate) struct Component {
    items: Vec<crate::Item>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self {
            items: props.value,
        }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <ul class="list-group">
            {
                for self.items.iter().map(|item| {
                    yew::html! {
                        <li class="list-group-item"><super::Item value=item /></li>
                    }
                })
            }
            </ul>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.items != props.value;
        self.items = props.value;

        should_render
    }
}

