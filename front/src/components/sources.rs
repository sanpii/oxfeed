#[derive(Clone)]
pub(crate) enum Message {
    Add,
    Cancel,
    Create(oxfeed_common::source::Entity),
    Error(String),
    Event(crate::Event),
    PageChange(usize),
    Update(crate::Pager<oxfeed_common::source::Entity>),
    NeedUpdate,
}

enum Scene {
    Add,
    View,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    filter: crate::Filter,
    scene: Scene,
    pager: Option<crate::Pager<oxfeed_common::source::Entity>>,
    pagination: oxfeed_common::Pagination,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Bridged;

        let props = ctx.props().clone();

        let callback = {
            let link = ctx.link().clone();
            move |e| link.send_message(Message::Event(e))
        };

        let component = Self {
            filter: props.filter,
            scene: Scene::View,
            pager: None,
            pagination: props.pagination,
            _producer: crate::event::Bus::bridge(std::rc::Rc::new(callback)),
        };

        ctx.link().send_message(Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        if matches!(msg, Message::Event(crate::Event::SourceUpdate)) {
            ctx.link().send_message(Message::NeedUpdate);
            return should_render;
        }

        match &self.scene {
            Scene::View => match msg {
                Message::Add => {
                    self.scene = Scene::Add;
                    should_render = true;
                }
                Message::Update(ref pager) => {
                    self.pager = Some(pager.clone());
                    should_render = true;
                }
                _ => (),
            },
            Scene::Add => match msg {
                Message::Cancel => {
                    self.scene = Scene::View;
                    should_render = true;
                }
                Message::Create(ref source) => crate::api!(
                    ctx.link(),
                    sources_create(source) -> |_| Message::NeedUpdate
                ),
                _ => (),
            },
        };

        if let Message::PageChange(page) = msg {
            self.pagination.page = page;
            gloo::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
            ctx.link().send_message(Message::NeedUpdate);
        } else if matches!(msg, Message::NeedUpdate) {
            self.scene = Scene::View;
            let pagination = &self.pagination;
            let filter = &self.filter;

            if filter.is_empty() {
                crate::api!(
                    ctx.link(),
                    sources_all(pagination) -> Message::Update
                );
            } else {
                crate::api!(
                    ctx.link(),
                    sources_search(filter, pagination) -> Message::Update
                );
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let add = match &self.scene {
            Scene::View => yew::html! {
                <a
                    class={ yew::classes!("btn", "btn-primary") }
                    title="Add"
                    onclick={ ctx.link().callback(|_| Message::Add) }
                >
                    <super::Svg icon="plus" size=24 />
                    { "Add" }
                </a>
            },
            Scene::Add => yew::html! {
                <ul class="list-group">
                    <li class="list-group-item">
                        <super::form::Source
                            source={ oxfeed_common::source::Entity::default() }
                            on_cancel={ ctx.link().callback(|_| Message::Cancel) }
                            on_submit={ ctx.link().callback(Message::Create) }
                        />
                    </li>
                </ul>
            },
        };

        let pager = match &self.pager {
            Some(pager) => pager,
            None => return add,
        };

        if pager.iterator.is_empty() {
            return add;
        }

        yew::html! {
            <>
                { add }
                <super::List<oxfeed_common::source::Entity>
                    value={ pager.clone() }
                    on_page_change={ ctx.link().callback(Message::PageChange) }
                />
            </>
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
        let props = ctx.props().clone();

        let should_render = self.pagination != props.pagination || self.filter != props.filter;

        if should_render {
            ctx.link().send_message(Message::NeedUpdate);
        }

        self.pagination = props.pagination;
        self.filter = props.filter;

        should_render
    }
}
