#[derive(Clone)]
pub(crate) enum Message {
    Event(crate::event::Event),
    NeedUpdate,
    Update(crate::Pager<crate::Item>),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Items(items) => Self::Update(items),
            crate::event::Api::SearchItems(items) => Self::Update(items),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: String,
    pub kind: String,
    pub pagination: crate::Pagination,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    kind: String,
    filter: String,
    link: yew::ComponentLink<Self>,
    pager: Option<crate::Pager<crate::Item>>,
    pagination: crate::Pagination,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl Component {
    fn fetch(&mut self) {
        if self.filter.is_empty() {
            self.api.items_all(&self.kind, &self.pagination);
        } else {
            self.api.search(&self.kind, &self.filter, &self.pagination);
        }
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::Bridged;

        let callback = link.callback(Self::Message::Event);

        let component = Self {
            api: crate::Api::new(link.clone()),
            kind: props.kind,
            filter: props.filter,
            link,
            pager: None,
            pagination: props.pagination,
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => {
                if matches!(event, crate::event::Event::ItemUpdate) {
                    self.link.send_message(Self::Message::NeedUpdate)
                }
            }
            Self::Message::NeedUpdate => self.fetch(),
            Self::Message::Update(pager) => self.pager = Some(pager),
        }

        true
    }

    fn view(&self) -> yew::Html {
        let pager = match &self.pager {
            Some(pager) => pager,
            None => return "Nothing found".into(),
        };

        if pager.iterator.is_empty() {
            return "Nothing found".into();
        }

        yew::html! {
            <super::List<crate::Item> value=pager />
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.filter != props.filter || self.pagination != props.pagination;

        if should_render {
            self.link.send_message(Self::Message::NeedUpdate);
        }

        self.filter = props.filter;
        self.pagination = props.pagination;

        should_render
    }
}
