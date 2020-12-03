#[derive(Clone)]
pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Save(crate::Source),
    Saved(crate::Source),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::SourceDelete(_) => Self::Deleted,
            crate::event::Api::SourceUpdate(source) => Self::Saved(source),
            _ => unreachable!(),
        }
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
    api: crate::Api<Self>,
    scene: Scene,
    link: yew::ComponentLink<Self>,
    source: crate::Source,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            api: crate::Api::new(link.clone()),
            scene: Scene::View,
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
                        self.api
                            .sources_delete(self.source.source_id.as_ref().unwrap());
                    }
                }
                Self::Message::Deleted => {
                    let parent = self.link.get_parent().unwrap();
                    let sources = parent.clone().downcast::<super::Sources>();

                    sources.send_message(super::sources::Message::NeedUpdate);
                }
                Self::Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                }
                _ => unreachable!(),
            },
            Scene::Edit => match msg {
                Self::Message::Cancel => {
                    self.scene = Scene::View;
                    return true;
                }
                Self::Message::Save(source) => {
                    self.source = source;
                    self.api
                        .sources_update(self.source.source_id.as_ref().unwrap(), &self.source);
                    return true;
                }
                Self::Message::Saved(source) => {
                    self.source = source;
                    self.scene = Scene::View;
                    return true;
                }
                _ => unreachable!(),
            },
        }

        false
    }

    fn view(&self) -> yew::Html {
        match &self.scene {
            Scene::Edit => yew::html! {
                <super::form::Source
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
                            { source.title.as_ref().unwrap_or(&source.url) }
                            {
                                if let Some(last_error) = source.last_error {
                                    yew::html! {
                                        <>
                                            { " · " }
                                            <span class="error">{ last_error }</span>
                                        </>
                                    }
                                }
                                else {
                                    "".into()
                                }
                            }
                        </div>

                        <div class=("btn-group", "float-right")>
                            <button
                                class=("btn", "btn-primary")
                                title="Edit"
                                onclick=self.link.callback(move |_| Message::Edit)
                            >
                                <super::Svg icon="pencil-square" size=16 />
                            </button>
                            <button
                                class=("btn", "btn-danger")
                                title="Delete"
                                onclick=self.link.callback(|_| Message::Delete)
                            >
                                <super::Svg icon="trash" size=16 />
                            </button>
                        </div>

                        <div class="tags">
                        {
                            for source.tags.iter().map(|tag| {
                                yew::html! { <super::Tag value=tag /> }
                            })
                        }
                        </div>
                    </>
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.source != props.value;

        self.source = props.value;

        should_render
    }
}
