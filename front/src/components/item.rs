pub enum Message {
    Content(String),
    Error(String),
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

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub value: oxfeed_common::item::Item,
}

pub struct Component {
    content: Option<String>,
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    scene: Scene,
    item: oxfeed_common::item::Item,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            content: None,
            event_bus: crate::event::Bus::dispatcher(),
            item: ctx.props().value.clone(),
            scene: Scene::Hidden,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Error(_) => (),
            Message::Content(content) => self.content = Some(content),
            Message::ToggleContent => {
                self.scene = !self.scene;

                if self.scene == Scene::Expanded && self.content.is_none() {
                    let item_id = &self.item.id;

                    crate::api!(
                        ctx.link(),
                        items_content(item_id) -> Message::Content
                    );
                }
            }
            Message::ToggleFavorite => {
                let item_id = &self.item.id;
                let key = "favorite";
                let value = !self.item.favorite;

                crate::api!(
                    ctx.link(),
                    items_tag(item_id, key, value) -> |_| Message::Toggled
                );
            }
            Message::ToggleRead => {
                let item_id = &self.item.id;
                let key = "read";
                let value = !self.item.read;

                crate::api!(
                    ctx.link(),
                    items_tag(item_id, key, value) -> |_| Message::Toggled
                );
            }
            Message::Toggled => self.event_bus.send(crate::Event::ItemUpdate),
        }

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let published_ago = chrono_humanize::HumanTime::from(self.item.published);

        let caret = match self.scene {
            Scene::Expanded => "chevron-up",
            Scene::Hidden => "chevron-down",
        };

        let title = gloo::utils::document().create_element("span").unwrap();
        title.set_inner_html(&self.item.title);

        let content = gloo::utils::document().create_element("div").unwrap();
        content.set_inner_html(self.content.as_ref().unwrap_or(&"Loading...".to_string()));

        let icon = if let Some(icon) = &self.item.icon {
            format!("{}{icon}", env!("API_URL"))
        } else {
            "/1px.png".to_string()
        };

        yew::html! {
            <>
                <img src={ icon } width="16" height="16" />
                <a href={ self.item.link.clone() } target="_blank">
                    { yew::virtual_dom::VNode::VRef(title.into()) }
                </a>
                {
                    for self.item.tags.iter().map(|tag| {
                        yew::html! { <super::Tag value={ tag.clone() } /> }
                    })
                }
                <span class="text-muted">{ "· " }{ &self.item.source }</span>
                <div class="float-end">
                    <span class="text-muted">{ &published_ago }</span>
                    <span onclick={ ctx.link().callback(|_| Message::ToggleContent) }>
                        <super::Svg icon={ caret } size=24 />
                    </span>
                </div>
                <div class="float-end">
                    {
                        if self.scene == Scene::Hidden {
                            yew::html! {
                                <super::Actions
                                    inline=true
                                    read={ self.item.read }
                                    on_read={ ctx.link().callback(|_| Message::ToggleRead) }
                                    favorite={ self.item.favorite }
                                    on_favorite={ ctx.link().callback(|_| Message::ToggleFavorite) }
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
                                    read={ self.item.read }
                                    on_read={ ctx.link().callback(|_| Message::ToggleRead) }
                                    favorite={ self.item.favorite }
                                    on_favorite={ ctx.link().callback(|_| Message::ToggleFavorite) }
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

    fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
        let should_render = self.item != ctx.props().value;

        self.item = ctx.props().value.clone();

        if should_render {
            self.content = None;
            self.scene = Scene::Hidden;
        }

        should_render
    }
}
