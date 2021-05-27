#[derive(Clone)]
pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    ToggleActive(bool),
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
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            api: crate::Api::new(link.clone()),
            scene: Scene::View,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        if let Self::Message::Saved(source) = msg {
            self.props.value = source;
            self.scene = Scene::View;
            return true;
        }

        match self.scene {
            Scene::View => match msg {
                Self::Message::Delete => {
                    let message =
                        format!("Would you like delete '{}' source?", self.props.value.title);

                    if yew::services::dialog::DialogService::confirm(&message) {
                        self.api.sources_delete(&self.props.value.id.unwrap());
                    }
                }
                Self::Message::Deleted => (),
                Self::Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                }
                Self::Message::ToggleActive(active) => {
                    self.props.value.active = active;
                    self.api
                        .sources_update(&self.props.value.id.unwrap(), &self.props.value);
                    return true;
                }
                _ => (),
            },
            Scene::Edit => match msg {
                Self::Message::Cancel => {
                    self.scene = Scene::View;
                    return true;
                }
                Self::Message::Save(source) => {
                    self.props.value = source;
                    self.api
                        .sources_update(&self.props.value.id.unwrap(), &self.props.value);
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
                    source=self.props.value.clone()
                    on_cancel=self.link.callback(|_| Message::Cancel)
                    on_submit=self.link.callback(|source| Message::Save(source))
                />
            },
            Scene::View => {
                let source = self.props.value.clone();

                yew::html! {
                    <>
                        <div class="d-inline-flex">
                            <super::Switch
                                id=format!("active-{}", source.id.unwrap_or_default().to_string())
                                active=source.active
                                on_toggle=self.link.callback(Self::Message::ToggleActive)
                            />

                            { source.title }

                            {
                                if let Some(last_error) = source.last_error {
                                    yew::html! {
                                        <super::Error text=last_error />
                                    }
                                }
                                else {
                                    "".into()
                                }
                            }
                        </div>

                        <div class=yew::classes!("btn-group", "float-end")>
                            {
                                if !source.webhooks.is_empty() {
                                    yew::html! {
                                        <button class=yew::classes!("btn", "btn-warning") disabled=true>
                                            <super::Svg icon="plug" size=16 />
                                        </button>
                                    }
                                } else {
                                    "".into()
                                }
                            }
                            <button
                                class=yew::classes!("btn", "btn-primary")
                                title="Edit"
                                onclick=self.link.callback(move |_| Message::Edit)
                            >
                                <super::Svg icon="pencil-square" size=16 />
                            </button>
                            <button
                                class=yew::classes!("btn", "btn-danger")
                                title="Delete"
                                onclick=self.link.callback(|_| Message::Delete)
                            >
                                <super::Svg icon="trash" size=16 />
                            </button>
                        </div>

                        <div class="tags">
                        {
                            for source.tags.iter().map(|tag| {
                                yew::html! { <super::Tag value=tag.clone() /> }
                            })
                        }
                        </div>
                    </>
                }
            }
        }
    }

    crate::change!(props.value);
}
