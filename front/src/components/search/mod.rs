mod bar;

pub(crate) enum Message {
    Event(crate::event::Event),
}

pub(crate) use bar::Component as Bar;

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub kind: String,
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    kind: String,
    pagination: oxfeed_common::Pagination,
    term: String,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let callback = link.callback(Self::Message::Event);

        Self {
            pagination: props.pagination,
            kind: props.kind,
            term: Self::term(),
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => {
                if let crate::event::Event::Redirected(_) = event {
                    self.term = Self::term();
                    return true;
                }
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        match self.kind.as_str() {
            "sources" => yew::html! {
                <super::Sources filter=self.term.clone() pagination=self.pagination />
            },
            "all" => yew::html! {
                <super::Items kind="all" filter=self.term.clone() pagination=self.pagination />
            },
            "favorites" => yew::html! {
                <super::Items kind="favorites" filter=self.term.clone() pagination=self.pagination />
            },
            "unread" => yew::html! {
                <super::Items kind="unread" filter=self.term.clone() pagination=self.pagination />
            },
            _ => unreachable!(),
        }
    }

    crate::change!(kind, pagination);
}

impl Component {
    fn term() -> String {
        let location = crate::Location::new();
        location.query().get("q").cloned().unwrap_or_default()
    }
}
