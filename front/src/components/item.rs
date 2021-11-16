pub(crate) enum Message {
    Error(oxfeed_common::Error),
    Content(String),
    ToggleContent,
    ToggleRead,
    ToggleFavorite,
    Toggled,
}

#[derive(Clone, Copy, PartialEq)]
enum Scene {
    Hidden,
    Expanded,
}

impl std::ops::Not for Scene {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Expanded => Self::Hidden,
            Self::Hidden => Self::Expanded,
        }
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub value: oxfeed_common::item::Item,
}

pub(crate) struct Component {
    content: Option<String>,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    item: oxfeed_common::item::Item,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::Dispatched;

        Self {
            content: None,
            event_bus: crate::event::Bus::dispatcher(),
            item: props.value,
            link,
            scene: Scene::Hidden,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::Error(err) => self.event_bus.send(err.into()),
            Message::Content(content) => self.content = Some(content),
            Message::ToggleContent => {
                self.scene = !self.scene;

                if self.scene == Scene::Expanded && self.content.is_none() {
                    let item_id = &self.item.id;

                    crate::api!(
                        self.link,
                        items_content(item_id) -> Message::Content, Message::Error
                    );
                }
            }
            Message::ToggleFavorite => {
                let item_id = &self.item.id;
                let key = "favorite";
                let value = !self.item.favorite;

                crate::api!(
                    self.link,
                    items_tag(item_id, key, value) -> |_| Message::Toggled, Message::Error
                );
            }
            Message::ToggleRead => {
                let item_id = &self.item.id;
                let key = "read";
                let value = !self.item.read;

                crate::api!(
                    self.link,
                    items_tag(item_id, key, value) -> |_| Message::Toggled, Message::Error
                );
            }
            Message::Toggled => self.event_bus.send(crate::Event::ItemUpdate),
        }

        true
    }

    fn view(&self) -> yew::Html {
        let published_ago = chrono_humanize::HumanTime::from(self.item.published);

        let caret = match self.scene {
            Scene::Expanded => "chevron-up",
            Scene::Hidden => "chevron-down",
        };

        let title = yew::utils::document().create_element("span").unwrap();
        title.set_inner_html(&self.item.title);

        let content = yew::utils::document().create_element("div").unwrap();
        content.set_inner_html(self.content.as_ref().unwrap_or(&"Loading...".to_string()));

        let icon = if let Some(icon) = &self.item.icon {
            format!("{}{}", env!("API_URL"), icon)
        } else {
            "/1px.png".to_string()
        };

        yew::html! {
            <>
                <img src=icon width="16" height="16" />
                <a href=self.item.link.clone() target="_blank">
                    { yew::virtual_dom::VNode::VRef(title.into()) }
                </a>
                {
                    for self.item.tags.iter().map(|tag| {
                        yew::html! { <super::Tag value=tag.clone() /> }
                    })
                }
                <span class="text-muted">{ "· " }{ &self.item.source }</span>
                <div class="float-end">
                    <span class="text-muted">{ &published_ago }</span>
                    <span onclick=self.link.callback(|_| Message::ToggleContent)>
                        <super::Svg icon=caret size=24 />
                    </span>
                </div>
                <div class="float-end">
                    {
                        if self.scene == Scene::Hidden {
                            yew::html! {
                                <super::Actions
                                    inline=true
                                    read=self.item.read
                                    on_read=self.link.callback(|_| Message::ToggleRead)
                                    favorite=self.item.favorite
                                    on_favorite=self.link.callback(|_| Message::ToggleFavorite)
                                />
                            }
                        } else {
                            "".into()
                        }
                    }
                    {
                        if self.scene == Scene::Hidden && self.item.favorite {
                            yew::html! {
                                <div class="favorite">
                                    <super::Svg icon="star-fill" size=24 />
                                </div>
                            }
                        } else {
                            "".into()
                        }
                    }
                </div>
                {
                    if self.scene == Scene::Expanded {
                        yew::html! {
                            <>
                                { yew::virtual_dom::VNode::VRef(content.into()) }

                                <hr />
                                <super::Actions
                                    read=self.item.read
                                    on_read=self.link.callback(|_| Message::ToggleRead)
                                    favorite=self.item.favorite
                                    on_favorite=self.link.callback(|_| Message::ToggleFavorite)
                                />
                            </>
                        }
                    } else {
                        "".into()
                    }
                }
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.item != props.value;

        self.item = props.value;

        if should_render {
            self.content = None;
            self.scene = Scene::Hidden;
        }

        should_render
    }
}
