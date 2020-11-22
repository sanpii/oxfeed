#[derive(Clone)]
pub(crate) enum Message {
    Error(String),
    NeedUpdate,
    Update(Vec<crate::Item>),
    Updated(crate::Item),
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = ();

    fn try_from((_, response): (http::Method, yew::format::Text)) -> Result<Self, ()> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match serde_json::from_str(&data) {
            Ok(sources) => Self::Update(sources),
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        Ok(message)
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: String,
    #[prop_or_default]
    pub on_update: yew::Callback<crate::Item>,
}

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    url: String,
    items: Vec<crate::Item>,
    link: yew::ComponentLink<Self>,
    on_update: yew::Callback<crate::Item>,
}

impl Component {
    fn url(filter: &str) -> String {
        if filter == "all" {
            "/items/".to_string()
        } else {
            format!("/items/{}", filter)
        }
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let items = Vec::new();
        let url = Self::url(&props.filter);
        let fetch_task = crate::get(&link, &url, yew::format::Nothing).ok();

        Self {
            fetch_task,
            items,
            link,
            url,
            on_update: props.on_update,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Error(error) => {
                log::error!("{:?}", error);
                return false;
            },
            Self::Message::NeedUpdate => {
                self.fetch_task = crate::get(&self.link, &self.url, yew::format::Nothing).ok();
                return false;
            },
            Self::Message::Update(ref items) => self.items = items.clone(),
            Self::Message::Updated(item) => self.on_update.emit(item),
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <ul class="list-group">
            {
                for self.items.iter().map(|item| {
                    yew::html! {
                        <li class="list-group-item">
                            <super::Item
                                value=item
                                on_read=self.link.callback(|e| Self::Message::Updated(e))
                            />
                        </li>
                    }
                })
            }
            </ul>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let url = Self::url(&props.filter);
        let should_render = self.url != url;

        self.url = url;
        self.on_update = props.on_update;

        should_render
    }
}
