#[derive(Clone)]
pub enum Message {
    Error(String),
    Event(crate::Event),
    NeedUpdate,
    PageChange(usize),
    Update(crate::Pager<oxfeed_common::item::Item>),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
    pub kind: String,
    pub pagination: oxfeed_common::Pagination,
}

pub struct Component {
    kind: String,
    filter: crate::Filter,
    pager: Option<crate::Pager<oxfeed_common::item::Item>>,
    pagination: oxfeed_common::Pagination,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Bridged;

        let props = ctx.props().clone();
        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(Message::Event(e))
        };

        let component = Self {
            kind: props.kind,
            filter: props.filter,
            pager: None,
            pagination: props.pagination,
            _producer: crate::event::Bus::bridge(std::rc::Rc::new(callback)),
        };

        ctx.link().send_message(Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Error(_) => (),
            Message::Event(event) => {
                if matches!(event, crate::Event::ItemUpdate) {
                    ctx.link().send_message(Message::NeedUpdate);
                }
            }
            Message::PageChange(page) => {
                self.pagination.page = page;
                gloo::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
                ctx.link().send_message(Message::NeedUpdate);
            }
            Message::NeedUpdate => {
                let filter = &self.filter;
                let kind = &self.kind;
                let pagination = self.pagination;

                if filter.is_empty() {
                    crate::api!(
                        ctx.link(),
                        items_all(kind, pagination) -> Message::Update
                    );
                } else {
                    crate::api!(
                        ctx.link(),
                        items_search(kind, filter, pagination) -> Message::Update
                    );
                }
            }
            Message::Update(pager) => {
                self.pager = Some(pager);
                should_render = true;
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let Some(pager) = &self.pager else {
            return yew::html! {
                <super::Empty />
            }
        };

        yew::html! {
            <super::List<oxfeed_common::item::Item>
                value={ pager.clone() }
                on_page_change={ ctx.link().callback(Message::PageChange) }
            />
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
        let props = ctx.props().clone();

        let should_render = self.kind != props.kind
            || self.filter != props.filter
            || self.pagination != props.pagination;

        if should_render {
            ctx.link().send_message(Message::NeedUpdate);
        }

        self.kind = props.kind;
        self.filter = props.filter;
        self.pagination = props.pagination;

        should_render
    }
}
