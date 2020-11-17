#[derive(Clone)]
pub(crate) enum Message {
    Error(String),
    Update(Vec<crate::Source>),
    NeedUpdate,
    Nothing,
}

impl std::convert::TryFrom<yew::format::Text> for Message {
    type Error = ();

    fn try_from(response: yew::format::Text) -> Result<Self, ()> {
        let message = match response {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(sources) => Self::Update(sources),
                Err(err) => Self::Error(err.to_string()),
            },
            Err(err) => Self::Error(err.to_string()),
        };

        Ok(message)
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
        let fetch_task = crate::get(&link, "/sources/", yew::format::Nothing, Message::Nothing).ok();

        Self {
            fetch_task,
            link,
            sources,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Update(sources) => self.sources = sources,
            Self::Message::Error(error) => crate::console::error(&error),
            Self::Message::NeedUpdate => self.fetch_task = crate::get(&self.link, "/sources/", yew::format::Nothing, Message::Nothing).ok(),
            Self::Message::Nothing => (),
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
