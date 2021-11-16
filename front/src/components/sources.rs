#[derive(Clone)]
pub(crate) enum Message {
    Add,
    Cancel,
    Create(oxfeed_common::source::Entity),
    Event(crate::Event),
    PageChange(usize),
    Update(crate::Pager<oxfeed_common::source::Entity>),
    NeedUpdate,
}

enum Scene {
    Add,
    View,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    filter: crate::Filter,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    pager: Option<crate::Pager<oxfeed_common::source::Entity>>,
    pagination: oxfeed_common::Pagination,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let callback = link.callback(Message::Event);

        let component = Self {
            filter: props.filter,
            link,
            scene: Scene::View,
            pager: None,
            pagination: props.pagination,
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.send_message(Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match &self.scene {
            Scene::View => match msg {
                Message::Add => self.scene = Scene::Add,
                Message::Update(ref pager) => self.pager = Some(pager.clone()),
                _ => (),
            },
            Scene::Add => match msg {
                Message::Cancel => self.scene = Scene::View,
                Message::Create(ref source) => {
                    crate::api!(
                        self.link,
                        sources_create(source) -> |_| Message::NeedUpdate
                    );
                }
                _ => (),
            },
        };

        if let Message::PageChange(page) = msg {
            self.pagination.page = page;
            yew::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
            self.link.send_message(Message::NeedUpdate);

            return false;
        } else if matches!(msg, Message::NeedUpdate) {
            self.scene = Scene::View;
            let pagination = &self.pagination;
            let filter = &self.filter;

            if filter.is_empty() {
                crate::api!(
                    self.link,
                    sources_all(pagination) -> Message::Update
                );
            } else {
                crate::api!(
                    self.link,
                    sources_search(filter, pagination) -> Message::Update
                );
            }

            return false;
        }

        true
    }

    fn view(&self) -> yew::Html {
        let add = match &self.scene {
            Scene::View => yew::html! {
                <a
                    class=yew::classes!("btn", "btn-primary")
                    title="Add"
                    onclick=self.link.callback(|_| Message::Add)
                >
                    <super::Svg icon="plus" size=24 />
                    { "Add" }
                </a>
            },
            Scene::Add => yew::html! {
                <ul class="list-group">
                    <li class="list-group-item">
                        <super::form::Source
                            source=oxfeed_common::source::Entity::default()
                            on_cancel=self.link.callback(|_| Message::Cancel)
                            on_submit=self.link.callback(Message::Create)
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
                    value=pager.clone()
                    on_page_change=self.link.callback(Message::PageChange)
                />
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.pagination != props.pagination || self.filter != props.filter;

        if should_render {
            self.link.send_message(Message::NeedUpdate);
        }

        self.pagination = props.pagination;
        self.filter = props.filter;

        should_render
    }
}
