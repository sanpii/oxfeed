pub enum Message {
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

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub value: oxfeed_common::source::Entity,
}

pub struct Component {
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    scene: Scene,
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            scene: Scene::View,
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        if let Message::Saved(source) = msg {
            self.props.value = source;
            self.scene = Scene::View;
            return true;
        }

        let mut should_render = false;

        match self.scene {
            Scene::View => match msg {
                Message::Delete => {
                    let message =
                        format!("Would you like delete '{}' source?", self.props.value.title);

                    if gloo::dialogs::confirm(&message) {
                        let id = self.props.value.id.unwrap();

                        crate::api!(
                            ctx.link(),
                            sources_delete(id) -> |_| Message::Deleted
                        );
                    }
                }
                Message::Deleted => self.event_bus.send(crate::Event::SourceUpdate),
                Message::Edit => {
                    self.scene = Scene::Edit;
                    should_render = true;
                }
                Message::Saved(_) => self.event_bus.send(crate::Event::SourceUpdate),
                Message::ToggleActive(active) => {
                    let value = &mut self.props.value;
                    let id = &value.id.unwrap();

                    value.active = active;

                    crate::api!(
                        ctx.link(),
                        sources_update(id, value) -> Message::Saved
                    );

                    should_render = true;
                }
                _ => (),
            },
            Scene::Edit => match msg {
                Message::Cancel => {
                    self.scene = Scene::View;
                    should_render = true;
                }
                Message::Save(source) => {
                    let id = &self.props.value.id.unwrap();

                    self.props.value = source.clone();

                    crate::api!(
                        ctx.link(),
                        sources_update(id, source) -> Message::Saved
                    );

                    should_render = true;
                }
                _ => unreachable!(),
            },
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.scene {
            Scene::Edit => yew::html! {
                <super::form::Source
                    source={ self.props.value.clone() }
                    on_cancel={ ctx.link().callback(|_| Message::Cancel) }
                    on_submit={ ctx.link().callback(Message::Save) }
                />
            },
            Scene::View => {
                let source = self.props.value.clone();

                yew::html! {
                    <>
                        <div class="d-inline-flex">
                            <super::Switch
                                id={ format!("active-{}", source.id.unwrap_or_default()) }
                                active={ source.active }
                                on_toggle={ ctx.link().callback(Message::ToggleActive) }
                            />

                            { source.title }

                            {
                                if let Some(last_error) = source.last_error {
                                    yew::html! {
                                        <super::Error text={ last_error } />
                                    }
                                }
                                else {
                                    "".into()
                                }
                            }
                        </div>

                        <div class={ yew::classes!("btn-group", "float-end") }>
                            {
                                if source.webhooks.is_empty() {
                                    "".into()
                                } else {
                                    yew::html! {
                                        <button class={ yew::classes!("btn", "btn-warning") } disabled=true>
                                            <super::Svg icon="plug" size=16 />
                                        </button>
                                    }
                                }
                            }
                            <button
                                class={ yew::classes!("btn", "btn-primary") }
                                title="Edit"
                                onclick={ ctx.link().callback(move |_| Message::Edit) }
                            >
                                <super::Svg icon="pencil-square" size=16 />
                            </button>
                            <button
                                class={ yew::classes!("btn", "btn-danger") }
                                title="Delete"
                                onclick={ ctx.link().callback(|_| Message::Delete) }
                            >
                                <super::Svg icon="trash" size=16 />
                            </button>
                        </div>

                        <div class="tags">
                        {
                            for source.tags.iter().map(|tag| {
                                yew::html! { <super::Tag value={ tag.clone() } /> }
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
