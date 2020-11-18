#[derive(Clone, Debug)]
pub(crate) enum Message {
    Error(String),
    NeedUpdate,
    Nothing,
    Update(Vec<crate::Item>),
}

impl std::convert::TryFrom<yew::format::Text> for Message {
    type Error = ();

    fn try_from(response: yew::format::Text) -> Result<Self, ()> {
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

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    items: Vec<crate::Item>,
    link: yew::ComponentLink<Self>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let items = Vec::new();
        let fetch_task = crate::get(&link, "/items/unread", yew::format::Nothing, Message::Nothing).ok();

        Self {
            fetch_task,
            items,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Error(error) => log::error!("{:?}", error),
            Self::Message::NeedUpdate => {
                self.fetch_task = crate::get(&self.link, "/items/unread", yew::format::Nothing, Message::Nothing).ok();
                return false;
            },
            Self::Message::Nothing => return false,
            Self::Message::Update(ref items) => self.items = items.clone(),
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <super::Items value=self.items.clone() />
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
