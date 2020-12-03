#[derive(Clone, Debug)]
pub(crate) enum Message {
    Content(String),
    ToggleContent,
    ToggleRead,
    ToggleFavorite,
    Toggled,
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::ItemContent(content) => Self::Content(content),
            crate::event::Api::ItemPatch => Self::Toggled,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    api: crate::Api<Self>,
    content: Option<String>,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    item: oxfeed_common::item::Item,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            api: crate::Api::new(link.clone()),
            content: None,
            item: props.value,
            link,
            scene: Scene::Hidden,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Content(content) => self.content = Some(content),
            Self::Message::ToggleContent => {
                self.scene = !self.scene;

                if self.scene == Scene::Expanded && self.content.is_none() {
                    self.api.items_content(&self.item.item_id);
                }
            }
            Self::Message::ToggleFavorite => {
                self.api
                    .items_tag(&self.item.item_id, "favorite", !self.item.favorite)
            }
            Self::Message::ToggleRead => {
                self.api
                    .items_tag(&self.item.item_id, "read", !self.item.read)
            }
            Self::Message::Toggled => (),
        }

        true
    }

    fn view(&self) -> yew::Html {
        // @FIXME https://gitlab.com/imp/chrono-humanize-rs/-/merge_requests/5
        let published_ago =
            chrono_humanize::HumanTime::from(self.item.published - chrono::Utc::now());

        let caret = match self.scene {
            Scene::Expanded => "caret-down",
            Scene::Hidden => "caret-up",
        };

        let content = yew::utils::document().create_element("div").unwrap();
        content.set_inner_html(&self.content.as_ref().unwrap_or(&"Loading...".to_string()));

        let icon = format!(
            "{}/items/{}/icon?token={}",
            env!("API_URL"),
            self.item.item_id,
            crate::Api::<Self>::token()
        );

        yew::html! {
            <>
                <img src=icon width="16" height="16" />
                <a href=self.item.link.clone() target="_blank">{ &self.item.title }</a>
                {
                    for self.item.tags.iter().map(|tag| {
                        yew::html! { <super::Tag value=tag /> }
                    })
                }
                <span class="text-muted">{ " · " }{ &self.item.source }</span>
                <div class="float-right">
                    <span class="text-muted">{ &published_ago }</span>
                    <span onclick=self.link.callback(|_| Message::ToggleContent)>
                        <super::Svg icon=caret size=24 />
                    </span>
                </div>
                <div class="float-right">
                    {
                        if self.scene == Scene::Hidden {
                            yew::html! {
                                <super::Actions
                                    inline=true
                                    read=self.item.read
                                    on_read=self.link.callback(|_| Self::Message::ToggleRead)
                                    favorite=self.item.favorite
                                    on_favorite=self.link.callback(|_| Self::Message::ToggleFavorite)
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
                                    on_read=self.link.callback(|_| Self::Message::ToggleRead)
                                    favorite=self.item.favorite
                                    on_favorite=self.link.callback(|_| Self::Message::ToggleFavorite)
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

        should_render
    }
}
