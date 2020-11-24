#[derive(Clone, Debug)]
pub(crate) enum Message {
    Content(String),
    Error(String),
    ToggleContent,
    ToggleRead,
    ToggleFavorite,
    Update(crate::Item),
    Toggled,
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = ();

    fn try_from((method, response): (http::Method, yew::format::Text)) -> Result<Self, ()> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match method {
            http::Method::GET => Message::Content(data),
            http::Method::PATCH => Message::Toggled,
            http::Method::POST => Message::Update(serde_json::from_str(&data).map_err(|_| ())?),
            _ => return Err(()),
        };

        Ok(message)
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
    pub value: crate::Item,
}

pub(crate) struct Component {
    content: Option<String>,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    fetch_task: Option<yew::services::fetch::FetchTask>,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    item: crate::Item,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            content: None,
            event_bus: crate::event::Bus::dispatcher(),
            fetch_task: None,
            item: props.value,
            link,
            scene: Scene::Hidden,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Content(content) => {
                self.fetch_task = None;
                self.content = Some(content);
            },
            Self::Message::Error(error) => log::error!("{}", error),
            Self::Message::ToggleContent => {
                self.scene = !self.scene;

                if self.scene == Scene::Expanded && self.content.is_none() {
                    self.fetch_task = crate::get(&self.link, &format!("/items/{}/content", self.item.item_id), yew::format::Nothing).ok();
                }
            },
            Self::Message::ToggleFavorite | Self::Message::ToggleRead => {
                let url = format!("/items/{}", self.item.item_id);
                let (key, value) = match msg {
                    Self::Message::ToggleFavorite => ("favorite", !self.item.favorite),
                    Self::Message::ToggleRead => ("read", !self.item.read),
                    _ => unreachable!(),
                };
                let json = serde_json::json!({
                    key: value,
                });

                self.fetch_task = crate::patch(&self.link, &url, yew::format::Json(&json)).ok();
            },
            Self::Message::Toggled =>  self.event_bus.send(crate::event::Message::ItemUpdate),
            Self::Message::Update(item) => self.item = item,
        }

        true
    }

    fn view(&self) -> yew::Html {
        // @FIXME https://gitlab.com/imp/chrono-humanize-rs/-/merge_requests/5
        let published_ago = chrono_humanize::HumanTime::from(self.item.published - chrono::Utc::now());

        let caret = match self.scene {
            Scene::Expanded => "caret-down",
            Scene::Hidden => "caret-up",
        };

        let content = yew::utils::document().create_element("div").unwrap();
        content.set_inner_html(&self.content.as_ref().unwrap_or(&"Loading...".to_string()));

        let icon = format!("{}/items/{}/icon", env!("API_URL"), self.item.item_id);

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
