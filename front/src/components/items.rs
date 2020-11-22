pub(crate) enum Message {
    Updated(crate::Item),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub value: Vec<crate::Item>,
    #[prop_or_default]
    pub on_update: yew::Callback<crate::Item>,
}

pub(crate) struct Component {
    items: Vec<crate::Item>,
    link: yew::ComponentLink<Self>,
    on_update: yew::Callback<crate::Item>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            items: props.value,
            on_update: props.on_update,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Updated(item) => self.on_update.emit(item),
        }

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <ul class="list-group">
            {
                for self.items.iter().map(|item| {
                    yew::html! {
                        <li class="list-group-item">
                            <super::Item
                                value=item
                                on_read=self.link.callback(|e| Self::Message::Updated(e))
                            />
                        </li>
                    }
                })
            }
            </ul>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.items != props.value;

        self.items = props.value;
        self.on_update = props.on_update;

        should_render
    }
}
