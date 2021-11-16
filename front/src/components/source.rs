pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Error(String),
    ToggleActive(bool),
    Save(oxfeed_common::source::Entity),
    Saved(oxfeed_common::source::Entity),
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
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    scene: Scene,
    link: yew::ComponentLink<Self>,
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            scene: Scene::View,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        if let Message::Saved(source) = msg {
            self.props.value = source;
            self.scene = Scene::View;
            return true;
        }

        match self.scene {
            Scene::View => match msg {
                Message::Delete => {
                    let message =
                        format!("Would you like delete '{}' source?", self.props.value.title);

                    if yew::services::dialog::DialogService::confirm(&message) {
                        let id = self.props.value.id.unwrap();

                        crate::api!(
                            self.link,
                            sources_delete(id) -> |_| Message::Deleted
                        );
                    }
                }
                Message::Deleted => self.event_bus.send(crate::Event::SourceUpdate),
                Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                }
                Message::Saved(_) => self.event_bus.send(crate::Event::SourceUpdate),
                Message::ToggleActive(active) => {
                    let value = &mut self.props.value;
                    let id = &value.id.unwrap();

                    value.active = active;

                    crate::api!(
                        self.link,
                        sources_update(id, value) -> Message::Saved
                    );

                    return true;
                }
                _ => (),
            },
            Scene::Edit => match msg {
                Message::Cancel => {
                    self.scene = Scene::View;
                    return true;
                }
                Message::Save(source) => {
                    let id = &self.props.value.id.unwrap();

                    self.props.value = source.clone();

                    crate::api!(
                        self.link,
                        sources_update(id, source) -> Message::Saved
                    );

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
                    on_submit=self.link.callback(Message::Save)
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
                                on_toggle=self.link.callback(Message::ToggleActive)
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
