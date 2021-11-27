mod bar;

pub(crate) enum Message {
    Event(crate::Event),
}

pub(crate) use bar::Component as Bar;

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub kind: String,
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    kind: String,
    pagination: oxfeed_common::Pagination,
    filter: crate::Filter,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Bridged;

        let props = ctx.props().clone();
        let callback = ctx.link().callback(Message::Event);

        Self {
            pagination: props.pagination,
            kind: props.kind,
            filter: crate::Filter::new(),
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        let Message::Event(event) = msg;

        if let crate::Event::Redirected(_) = event {
            self.filter = crate::Filter::new();

            true
        } else {
            false
        }
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        let filter = self.filter.clone();

        match self.kind.as_str() {
            "sources" => yew::html! {
                <super::Sources filter={ filter } pagination={ self.pagination } />
            },
            "all" => yew::html! {
                <super::Items kind="all" filter={ filter } pagination={ self.pagination } />
            },
            "favorites" => yew::html! {
                <super::Items kind="favorites" filter={ filter } pagination={ self.pagination } />
            },
            "unread" => yew::html! {
                <super::Items kind="unread" filter={ filter } pagination={ self.pagination } />
            },
            _ => unreachable!(),
        }
    }

    crate::change!(kind, pagination);
}
