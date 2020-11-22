pub(crate) enum Message {
    ToggleRead,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub inline: bool,
    pub read: bool,
    pub on_read: yew::Callback<()>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::ToggleRead => self.props.on_read.emit(()),
        }

        true
    }

    fn view(&self) -> yew::Html {
        let (label, eye) = if self.props.read {
            ("Mark as unread", "eye-slash")
        } else {
            ("Mark as read", "eye")
        };

        if self.props.inline {
            yew::html! {
                <div class=("actions", "inline")>
                    <span onclick=self.link.callback(|_| Self::Message::ToggleRead)>
                        <super::Svg icon=eye size=16 />
                    </span>
                </div>
            }
        } else {
            yew::html! {
                <div class="actions">
                    <button class=("btn", "btn-outline-secondary") onclick=self.link.callback(|_| Self::Message::ToggleRead)>
                        <super::Svg icon=eye size=24 />
                        { label }
                    </button>
                </div>
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.props != props;

        self.props = props;

        should_render
    }
}
