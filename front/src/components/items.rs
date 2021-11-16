#[derive(Clone)]
pub(crate) enum Message {
    Event(crate::Event),
    NeedUpdate,
    PageChange(usize),
    Update(crate::Pager<oxfeed_common::item::Item>),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
    pub kind: String,
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    kind: String,
    filter: crate::Filter,
    link: yew::ComponentLink<Self>,
    pager: Option<crate::Pager<oxfeed_common::item::Item>>,
    pagination: oxfeed_common::Pagination,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::Bridged;

        let callback = link.callback(Message::Event);

        let component = Self {
            kind: props.kind,
            filter: props.filter,
            link,
            pager: None,
            pagination: props.pagination,
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.send_message(Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::Event(event) => {
                if matches!(event, crate::Event::ItemUpdate) {
                    self.link.send_message(Message::NeedUpdate);
                }
            }
            Message::PageChange(page) => {
                self.pagination.page = page;
                yew::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
                self.link.send_message(Message::NeedUpdate);
            }
            Message::NeedUpdate => {
                let filter = &self.filter;
                let kind = &self.kind;
                let pagination = self.pagination;

                if filter.is_empty() {
                    crate::api!(
                        self.link,
                        items_all(kind, pagination) -> Message::Update
                    );
                } else {
                    crate::api!(
                        self.link,
                        items_search(kind, filter, pagination) -> Message::Update
                    );
                }
            }
            Message::Update(pager) => {
                self.pager = Some(pager);
                return true;
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        let pager = match &self.pager {
            Some(pager) => pager,
            None => {
                return yew::html! {
                    <super::Empty />
                }
            }
        };

        yew::html! {
            <super::List<oxfeed_common::item::Item>
                value=pager.clone()
                on_page_change=self.link.callback(Message::PageChange)
            />
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.kind != props.kind
            || self.filter != props.filter
            || self.pagination != props.pagination;

        if should_render {
            self.link.send_message(Message::NeedUpdate);
        }

        self.kind = props.kind;
        self.filter = props.filter;
        self.pagination = props.pagination;

        should_render
    }
}
