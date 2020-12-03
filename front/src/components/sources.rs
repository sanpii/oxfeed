#[derive(Clone)]
pub(crate) enum Message {
    Add,
    Cancel,
    Create(crate::Source),
    Event(crate::event::Event),
    Update(crate::Pager<crate::Source>),
    NeedUpdate,
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Sources(sources) => Self::Update(sources),
            crate::event::Api::SearchSources(sources) => Self::Update(sources),
            crate::event::Api::SourceCreate(sources) => Self::Create(sources),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Scene {
    Add,
    View,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: String,
    pub pagination: crate::Pagination,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    filter: String,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    pager: Option<crate::Pager<crate::Source>>,
    pagination: crate::Pagination,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let callback = link.callback(Self::Message::Event);

        let component = Self {
            api: crate::Api::new(link.clone()),
            filter: props.filter,
            link,
            scene: Scene::View,
            pager: None,
            pagination: props.pagination,
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match &self.scene {
            Scene::View => match msg {
                Self::Message::Add => self.scene = Scene::Add,
                Self::Message::Update(ref pager) => self.pager = Some(pager.clone()),
                _ => (),
            },
            Scene::Add => match msg {
                Self::Message::Cancel => self.scene = Scene::View,
                Self::Message::Create(ref source) => self.api.sources_create(source),
                _ => (),
            },
        };

        if let Self::Message::Event(ref event) = msg {
            if matches!(event, crate::event::Event::SourceUpdate) {
                self.link.send_message(Self::Message::NeedUpdate);
            }
        }

        if matches!(msg, Self::Message::NeedUpdate) {
            self.scene = Scene::View;

            if self.filter.is_empty() {
                self.api.sources_all(&self.pagination);
            } else {
                self.api.search("sources", &self.filter, &self.pagination);
            }

            return false;
        }

        true
    }

    fn view(&self) -> yew::Html {
        let add = match &self.scene {
            Scene::View => yew::html! {
                <a
                    class=("btn", "btn-primary")
                    title="Add"
                    onclick=self.link.callback(|_| Message::Add)
                >
                    <super::Svg icon="plus" size=24 />
                    { "Add" }
                </a>
            },
            Scene::Add => yew::html! {
                <super::form::Source
                    source=crate::Source::default()
                    oncancel=self.link.callback(|_| Message::Cancel)
                    onsubmit=self.link.callback(|source| Message::Create(source))
                />
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
                <super::List<crate::Source> value=pager />
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.pagination != props.pagination || self.filter != props.filter;

        if should_render {
            self.link.send_message(Self::Message::NeedUpdate);
        }

        self.pagination = props.pagination;
        self.filter = props.filter;

        should_render
    }
}
