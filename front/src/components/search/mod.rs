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

impl Component {
    fn filter(kind: &str, q: &str) -> String {
        format!("/search/{}?q={}", kind, q)
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let location = crate::Location::new();
        let term = location.query().get("q").cloned().unwrap_or_default();
        let callback = link.callback(|x| Self::Message::Event(x));

        Self {
            pagination: props.pagination,
            kind: props.kind,
            term,
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => match event {
                crate::event::Event::Search(term) => {
                    self.term = term;
                    return true;
                }
                _ => (),
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        let filter = Self::filter(&self.kind, &self.term);

        if self.kind == "sources" {
            yew::html! {
                <super::Sources filter=filter pagination=self.pagination />
            }
        } else {
            yew::html! {
                <super::Items filter=filter pagination=self.pagination />
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
