#[derive(Clone)]
pub(crate) enum Message {
    Add,
    Cancel,
    Create(crate::Source),
    Error(String),
    Update(crate::Pager<crate::Source>),
    NeedUpdate,
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = ();

    fn try_from((_, response): (http::Method, yew::format::Text)) -> Result<Self, ()> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match serde_json::from_str(&data) {
            Ok(pager) => Self::Update(pager),
            Err(_) => Self::NeedUpdate,
        };

        Ok(message)
    }
}

#[derive(Debug)]
enum Scene {
    Add,
    View,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub pagination: crate::Pagination,
    #[prop_or_default]
    pub filter: String,
}

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    filter: String,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    pager: Option<crate::Pager<crate::Source>>,
    pagination: crate::Pagination,
}

impl Component {
    fn create(&mut self, source: &crate::Source) {
        self.fetch_task = crate::post(&self.link, "/sources", source).ok();
    }

    fn url(&self) -> String {
        let mut url = if self.filter.is_empty() {
            "/sources".to_string()
        } else {
            self.filter.clone()
        };

        if !url.contains('?') {
            url.push('?');
        } else {
            url.push('&');
        }

        format!("{}page={}&limit={}", url, self.pagination.page, self.pagination.limit)
    }

    fn fetch(&mut self) -> Option<yew::services::fetch::FetchTask> {
        crate::get(&self.link, &self.url(), yew::format::Nothing).ok()
    }
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let mut component = Self {
            fetch_task: None,
            filter: props.filter,
            link,
            scene: Scene::View,
            pager: None,
            pagination: props.pagination,
        };

        component.fetch_task = component.fetch();

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        if let Self::Message::Error(error) = msg {
            log::error!("{:?}", error);
            return true;
        }

        match &self.scene {
            Scene::View => match msg {
                Self::Message::Add => self.scene = Scene::Add,
                Self::Message::Update(ref pager) => self.pager = Some(pager.clone()),
                _ => (),
            },
            Scene::Add => match msg {
                Self::Message::Cancel => self.scene = Scene::View,
                Self::Message::Create(ref source) => self.create(source),
                _ => (),
            },
        };

        if matches!(msg, Self::Message::NeedUpdate) {
            self.scene = Scene::View;
            self.fetch_task = self.fetch();
            return false;
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
                <super::Form
                    source=crate::Source::default()
                    oncancel=self.link.callback(|_| Message::Cancel)
                    onsubmit=self.link.callback(|source| Message::Create(source))
                />
            },
        };

        yew::html! {
            <>
                { add }
                <ul class="list-group">
                {
                    for pager.iterator.iter().map(|source| {
                        let class = if source.last_error.is_some() {
                            "list-group-item list-error"
                        } else {
                            "list-group-item"
                        };

                        yew::html! {
                            <li class=class><super::Source value=source /></li>
                        }
                    })
                }
                </ul>
                <super::Pager<crate::Source> base_url=self.filter.clone() value=pager />
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.pagination != props.pagination || self.filter != props.filter;

        self.pagination = props.pagination;
        self.filter = props.filter;
        self.link.send_message(Self::Message::NeedUpdate);

        should_render
    }
}
