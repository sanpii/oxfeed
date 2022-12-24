pub enum Message {
    ToggleFavorite,
    ToggleRead,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    #[prop_or_default]
    pub inline: bool,
    pub favorite: bool,
    pub read: bool,
    pub on_favorite: yew::Callback<()>,
    pub on_read: yew::Callback<()>,
}

pub struct Component {
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ToggleFavorite => self.props.on_favorite.emit(()),
            Message::ToggleRead => self.props.on_read.emit(()),
        }

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
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
                <div class={ yew::classes!("actions", "inline") }>
                    <span class="read" onclick={ ctx.link().callback(|_| Message::ToggleRead) } title={ read_label }>
                        <super::Svg icon={ eye } size=24 />
                    </span>
                    <span class="favorite" onclick={ ctx.link().callback(|_| Message::ToggleFavorite) } title={ favorite_label }>
                        <super::Svg icon={ star } size=24 />
                    </span>
                </div>
            }
        } else {
            yew::html! {
                <div class="actions">
                    <button class={ yew::classes!("btn", "btn-outline-secondary") } onclick={ ctx.link().callback(|_| Message::ToggleRead) }>
                        <super::Svg icon={ eye } size=24 />
                        { read_label }
                    </button>
                    <button class={ yew::classes!("btn", "btn-outline-warning") } onclick={ ctx.link().callback(|_| Message::ToggleFavorite) }>
                        <super::Svg icon={ star } size=24 />
                        { favorite_label }
                    </button>
                </div>
            }
        }
    }

    crate::change!(props);
}
