pub(crate) enum Message {
    ToggleFavorite,
    ToggleRead,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub inline: bool,
    pub favorite: bool,
    pub read: bool,
    pub on_favorite: yew::Callback<()>,
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
            Self::Message::ToggleFavorite => self.props.on_favorite.emit(()),
            Self::Message::ToggleRead => self.props.on_read.emit(()),
        }

        true
    }

    fn view(&self) -> yew::Html {
        let (read_label, eye) = if self.props.read {
            ("Mark as unread", "eye-slash")
        } else {
            ("Mark as read", "eye")
        };

        let (favorite_label, star) = if self.props.favorite {
            ("Removes from favorites", "star-fill")
        } else {
            ("Adds to favorites", "star")
        };

        if self.props.inline {
            yew::html! {
                <div class=("actions", "inline")>
                    <span onclick=self.link.callback(|_| Self::Message::ToggleRead) title=read_label>
                        <super::Svg icon=eye size=16 />
                    </span>
                    <span onclick=self.link.callback(|_| Self::Message::ToggleFavorite) title=favorite_label>
                        <super::Svg icon=star size=16 />
                    </span>
                </div>
            }
        } else {
            yew::html! {
                <div class="actions">
                    <button class=("btn", "btn-outline-secondary") onclick=self.link.callback(|_| Self::Message::ToggleRead)>
                        <super::Svg icon=eye size=24 />
                        { read_label }
                    </button>
                    <button class=("btn", "btn-outline-secondary") onclick=self.link.callback(|_| Self::Message::ToggleFavorite)>
                        <super::Svg icon=star size=24 />
                        { favorite_label }
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
