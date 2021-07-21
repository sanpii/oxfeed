#[derive(Clone)]
pub(crate) enum Message {
    Event(crate::event::Event),
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

impl Component {
    fn fetch(&mut self) {
        use yewtil::future::LinkFuture;

        let filter = self.filter.clone();
        let kind = self.kind.clone();
        let pagination = self.pagination;

        self.link.send_future(async move {
            if filter.is_empty() {
                crate::Api::items_all(&kind, &pagination)
                    .await
                    .map_or_else(|err| Message::Event(err.into()), Message::Update)
            } else {
                crate::Api::items_search(&kind, &filter, &pagination)
                    .await
                    .map_or_else(|err| Message::Event(err.into()), Message::Update)
            }
        });
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::Bridged;

        let callback = link.callback(Self::Message::Event);

        let component = Self {
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
                    self.link.send_message(Self::Message::NeedUpdate);
                }
            }
            Self::Message::PageChange(page) => {
                self.pagination.page = page;
                yew::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
                self.link.send_message(Self::Message::NeedUpdate);
            }
            Self::Message::NeedUpdate => self.fetch(),
            Self::Message::Update(pager) => {
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
                on_page_change=self.link.callback(Self::Message::PageChange)
            />
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.kind != props.kind
            || self.filter != props.filter
            || self.pagination != props.pagination;

        if should_render {
            self.link.send_message(Self::Message::NeedUpdate);
        }

        self.kind = props.kind;
        self.filter = props.filter;
        self.pagination = props.pagination;

        should_render
    }
}
