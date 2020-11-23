#[derive(Clone)]
pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Error(String),
    Save(crate::Source),
    Saved(crate::Source),
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = ();

    fn try_from((method, response): (http::Method, yew::format::Text)) -> Result<Self, ()> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match method {
            http::Method::DELETE => Self::Deleted,
            http::Method::PUT => Self::Saved(serde_json::from_str(&data).map_err(|_| ())?),
            _ => return Err(()),
        };

        Ok(message)
    }
}

enum Scene {
    Edit,
    View,
}

#[derive(yew::Properties, Clone)]
pub(crate) struct Properties {
    pub value: crate::Source,
}

pub(crate) struct Component {
    scene: Scene,
    fetch_task: Option<yew::services::fetch::FetchTask>,
    link: yew::ComponentLink<Self>,
    source: crate::Source,
}

impl Component {
    fn delete(&mut self) {
        self.fetch_task = crate::delete(&self.link, &format!("/sources/{}", self.source.source_id.as_ref().unwrap()), yew::format::Nothing).ok();
    }

    fn update(&mut self) {
        self.fetch_task = crate::put(&self.link, &format!("/sources/{}", self.source.source_id.as_ref().unwrap()), &self.source).ok();
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            scene: Scene::View,
            fetch_task: None,
            link,
            source: props.value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        if let Self::Message::Error(error) = msg {
            log::error!("{}", error);
            return false;
        }

        match self.scene {
            Scene::View => match msg {
                Self::Message::Delete => {
                    let name = self.source.title.as_ref().unwrap_or(&self.source.url);
                    let message = format!("Would you like delete '{}' source?", name);

                    if yew::services::dialog::DialogService::confirm(&message) {
                        self.delete();
                    }
                },
                Self::Message::Deleted => {
                    let parent = self.link.get_parent().unwrap();
                    let sources = parent.clone().downcast::<super::Sources>();

                    sources.send_message(super::sources::Message::NeedUpdate);
                },
                Self::Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                },
                _ => unreachable!(),
            },
            Scene::Edit => match msg {
                Self::Message::Cancel => {
                    self.scene = Scene::View;
                    return true;
                },
                Self::Message::Save(source) => {
                    self.source = source.clone();
                    self.update();
                    return true;
                },
                Self::Message::Saved(source) => {
                    self.source = source.clone();
                    self.scene = Scene::View;
                    return true;
                },
                _ => unreachable!(),
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        match &self.scene {
            Scene::Edit => yew::html! {
                <super::Form
                    source=self.source.clone()
                    oncancel=self.link.callback(|_| Message::Cancel)
                    onsubmit=self.link.callback(|source| Message::Save(source))
                />
            },
            Scene::View => {
                let source = self.source.clone();

                yew::html! {
                    <>
                        <div class="d-inline-flex">
                            <h4>{ source.title.as_ref().unwrap_or(&source.url) }</h4>
                        </div>

                        <div class=("btn-group", "float-right")>
                            <button
                                class=("btn", "btn-primary")
                                title="Edit"
                                onclick=self.link.callback(move |_| Message::Edit)
                            >
                                <super::Svg icon="pencil-square" size=24 />
                            </button>
                            <button
                                class=("btn", "btn-danger")
                                title="Delete"
                                onclick=self.link.callback(|_| Message::Delete)
                            >
                                <super::Svg icon="trash" size=24 />
                            </button>
                        </div>

                        <div class="tags">
                        {
                            for source.tags.iter().map(|tag| {
                                yew::html! {
                                    <span class=("badge", "badge-secondary")>{ tag }</span>
                                }
                            })
                        }
                        </div>
                    </>
                }
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
