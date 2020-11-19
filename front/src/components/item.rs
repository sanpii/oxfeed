#[derive(Clone)]
pub(crate) enum Message {
    Content(String),
    Error(String),
    Nothing,
    Toggle,
}

impl std::convert::TryFrom<yew::format::Text> for Message {
    type Error = ();

    fn try_from(response: yew::format::Text) -> Result<Self, ()> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        Ok(Message::Content(data))
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
    fetch_task: Option<yew::services::fetch::FetchTask>,
    item: crate::Item,
    link: yew::ComponentLink<Self>,
    scene: Scene,
    content: Option<String>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            link,
            item: props.value,
            scene: Scene::Hidden,
            content: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Content(content) => self.content = Some(content),
            Self::Message::Error(error) => log::error!("{}", error),
            Self::Message::Nothing => return false,
            Self::Message::Toggle => {
                self.scene = !self.scene;

                if self.scene == Scene::Expanded && self.content.is_none() {
                    self.fetch_task = crate::get(&self.link, &format!("/items/{}/content", self.item.item_id), yew::format::Nothing, Message::Nothing).ok();
                }
            }
        }

        true
    }

    fn view(&self) -> yew::Html {
        let empty_img = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7".to_string();
        // @FIXME https://gitlab.com/imp/chrono-humanize-rs/-/merge_requests/5
        let published_ago = chrono_humanize::HumanTime::from(self.item.published - chrono::Utc::now());

        let caret = match self.scene {
            Scene::Expanded => "caret-down",
            Scene::Hidden => "caret-up",
        };

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let content = document.create_element("div").unwrap();
        content.set_inner_html(&self.content.as_ref().unwrap_or(&"Loading...".to_string()));

        yew::html! {
            <>
                <img src=self.item.icon.as_ref().unwrap_or(&empty_img) width="16" height="16" />
                <a href=self.item.link.clone() target="_blank">{ &self.item.title }</a>
                <span class="text-muted">{ " · " }{ &self.item.source }</span>
                <div class="float-right">
                    <span class="text-muted">{ &published_ago }</span>
                    <span onclick=self.link.callback(|_| Message::Toggle)>
                        <super::Svg icon=caret size=24 />
                    </span>
                </div>
                {
                    if self.scene == Scene::Expanded {
                        yew::virtual_dom::VNode::VRef(content.into())
                    } else {
                        "".into()
                    }
                }
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
