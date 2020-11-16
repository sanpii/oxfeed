pub(crate) enum Message {
    Error(String),
    Update(Vec<crate::Source>),
    NeedUpdate,
}

impl From<yew::format::Text> for Message {
    fn from(response: yew::format::Text) -> Self {
        match response {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(sources) => Self::Update(sources),
                Err(err) => Self::Error(err.to_string()),
            },
            Err(err) => Self::Error(err.to_string()),
        }
    }
}

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    link: yew::ComponentLink<Self>,
    sources: Vec<crate::Source>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let sources = Vec::new();
        let fetch_task = crate::get(&link, "/sources/").ok();

        Self {
            fetch_task,
            link,
            sources,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Update(sources) => self.sources = sources,
            Self::Message::Error(error) => crate::console::error(&error.to_string()),
            Self::Message::NeedUpdate => self.fetch_task = crate::get(&self.link, "/sources/").ok(),
        };

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <ul class="list-group">
            {
                for self.sources.iter().map(|source| {
                    yew::html! {
                        <li class="list-group-item"><crate::components::Source value=source /></li>
                    }
                })
            }
            </ul>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
