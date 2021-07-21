pub(crate) enum Message {
    Cancel,
    Delete,
    Deleted,
    Edit,
    Error(oxfeed_common::Error),
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
        use yewtil::future::LinkFuture;

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
                        let id = self.props.value.id;

                        self.link.send_future(async move {
                            crate::Api::sources_delete(&id.unwrap())
                                .await
                                .map_or_else(Self::Message::Error, |_| Self::Message::Deleted)
                        });
                    }
                }
                Self::Message::Deleted => self.event_bus.send(crate::event::Event::SourceUpdate),
                Self::Message::Edit => {
                    self.scene = Scene::Edit;
                    return true;
                }
                Self::Message::Saved(_) => self.event_bus.send(crate::event::Event::SourceUpdate),
                Self::Message::ToggleActive(active) => {
                    let value = self.props.value.clone();

                    self.props.value.active = active;

                    self.link.send_future(async move {
                        crate::Api::sources_update(&value.id.unwrap(), &value)
                            .await
                            .map_or_else(Self::Message::Error, Self::Message::Saved)
                    });

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
                    let value = self.props.value.clone();

                    self.props.value = source;

                    self.link.send_future(async move {
                        crate::Api::sources_update(&value.id.unwrap(), &value)
                            .await
                            .map_or_else(Self::Message::Error, Self::Message::Saved)
                    });

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
