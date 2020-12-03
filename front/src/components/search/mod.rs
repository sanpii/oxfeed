mod bar;

pub(crate) enum Message {
    Event(crate::event::Event),
}

pub(crate) use bar::Component as Bar;

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub kind: String,
    pub pagination: crate::Pagination,
}

pub(crate) struct Component {
    kind: String,
    pagination: crate::Pagination,
    term: String,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let location = crate::Location::new();
        let term = location.query().get("q").cloned().unwrap_or_default();
        let callback = link.callback(Self::Message::Event);

        Self {
            pagination: props.pagination,
            kind: props.kind,
            term,
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => {
                if let crate::event::Event::Search(term) = event {
                    self.term = term;
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

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
