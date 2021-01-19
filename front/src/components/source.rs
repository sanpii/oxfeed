#[derive(Clone)]
pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    ToggleActive,
    Save(oxfeed_common::source::Entity),
    Saved(oxfeed_common::source::Entity),
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
    pub value: oxfeed_common::source::Entity,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    scene: Scene,
    link: yew::ComponentLink<Self>,
    source: oxfeed_common::source::Entity,
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
        if let Self::Message::Saved(source) = msg {
            self.source = source;
            self.scene = Scene::View;
            return true;
        }

        match self.scene {
            Scene::View => match msg {
                Self::Message::Delete => {
                    let message = format!("Would you like delete '{}' source?", self.source.title);

                    if yew::services::dialog::DialogService::confirm(&message) {
                        self.api.sources_delete(&self.source.source_id.unwrap());
                    }
                }
                Self::Message::Deleted => (),
                Self::Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                }
                Self::Message::ToggleActive => {
                    self.source.active = !self.source.active;
                    self.api
                        .sources_update(&self.source.source_id.unwrap(), &self.source);
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
                        .sources_update(&self.source.source_id.unwrap(), &self.source);
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
                    on_cancel=self.link.callback(|_| Message::Cancel)
                    on_submit=self.link.callback(|source| Message::Save(source))
                />
            },
            Scene::View => {
                let source = self.source.clone();

                yew::html! {
                    <>
                        <div class="d-inline-flex">
                            <div
                                class=("custom-control", "custom-switch")
                                title="active"
                                onclick=self.link.callback(|_| Self::Message::ToggleActive)
                            >
                                <input
                                    type="checkbox"
                                    class="custom-control-input"
                                    checked=source.active
                                />
                                <label class="custom-control-label" for="active"></label>
                            </div>

                            { source.title }

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
