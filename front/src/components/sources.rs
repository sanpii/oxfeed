#[derive(Debug, Clone)]
pub(crate) enum Message {
    Add,
    Cancel,
    Create(crate::Source),
    Error(String),
    Update(Vec<crate::Source>),
    NeedUpdate,
    Nothing,
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

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    sources: Vec<crate::Source>,
}

impl Component {
    fn create(&mut self, source: &crate::Source) {
        self.fetch_task = crate::post(&self.link, "/sources/", source, Message::Nothing).ok();
    }
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
            scene: Scene::View,
            sources,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        log::debug!("<Sources /> {:?} => {:?}", self.scene, msg);

        if let Self::Message::Error(error) = msg {
            log::error!("{:?}", error);
            return true;
        }

        match &self.scene {
            Scene::View => match msg {
                Self::Message::Add => self.scene = Scene::Add,
                Self::Message::Update(ref sources) => self.sources = sources.clone(),
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
            self.fetch_task = crate::get(&self.link, "/sources/", yew::format::Nothing, Message::Nothing).ok();
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
                    for self.sources.iter().map(|source| {
                        yew::html! {
                            <li class="list-group-item"><crate::components::Source value=source /></li>
                        }
                    })
                }
                </ul>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
