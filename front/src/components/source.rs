#[derive(Clone)]
pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Save(crate::Source),
    Saved,
}

impl std::convert::TryFrom<yew::format::Text> for Message {
    type Error = ();

    fn try_from(_: yew::format::Text) -> Result<Self, Self::Error> {
        Err(())
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
        self.fetch_task = crate::delete(&self.link, &format!("/sources/{}", self.source.source_id.as_ref().unwrap()), yew::format::Nothing, Message::Deleted).ok();
    }

    fn update(&mut self) {
        self.fetch_task = crate::put(&self.link, &format!("/sources/{}", self.source.source_id.as_ref().unwrap()), &self.source, Message::Saved).ok();
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
                Self::Message::Saved => {
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
                        { source.title.as_ref().unwrap_or(&source.url) }

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
                    </>
                }
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
