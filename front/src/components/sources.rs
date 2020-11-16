pub(crate) enum Message {
    Error(String),
    Update(Vec<crate::Source>),
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
    sources: Vec<crate::Source>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let sources = Vec::new();
        let fetch_task = crate::fetch(&link, "/sources/").ok();

        Self {
            fetch_task,
            sources,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Update(sources) => self.sources = sources,
            Self::Message::Error(error) => crate::console::error(&error.to_string()),
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
