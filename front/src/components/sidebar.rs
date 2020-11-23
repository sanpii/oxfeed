#[derive(Clone)]
pub(crate) enum Message {
    Error(String),
    Update(crate::Counts),
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = serde_json::Error;

    fn try_from((method, response): (http::Method, yew::format::Text)) -> Result<Self, serde_json::Error> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match method {
            http::Method::GET => Message::Update(serde_json::from_str(&data)?),
            _ => unreachable!(),
        };

        Ok(message)
    }
}

struct Link {
    count: usize,
    icon: &'static str,
    label: &'static str,
    url: &'static str,
}

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    links: Vec<Link>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let fetch_task = crate::get(&link, &"/counts", yew::format::Nothing).ok();

        let links = vec![
            Link {
                count: 0,
                icon: "collection",
                label: "All",
                url: "/",
            },
            Link {
                count: 0,
                icon: "book",
                label: "Unread",
                url: "/unread",
            },
            Link {
                count: 0,
                icon: "star",
                label: "Favorites",
                url: "/favorites",
            },
            Link {
                count: 0,
                icon: "diagram-3",
                label: "Sources",
                url: "/sources",
            },
        ];

        Self {
            fetch_task,
            links,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Error(error) => log::error!("{}", error),
            Self::Message::Update(counts) => {
                self.links[0].count = counts.all;
                self.links[1].count = counts.unread;
                self.links[2].count = counts.favorites;
                self.links[3].count = counts.sources;
                self.fetch_task = None;
                return true;
            },
        }

        false
    }

    fn view(&self) -> yew::Html {
        let router = yew_router::service::RouteService::<()>::new();
        let current_url = router.get_path();

        yew::html! {
            <ul class="nav flex-column">
            {
                for self.links.iter().map(|link| yew::html! {
                    <li class="nav-item">
                        <a
                            href={ link.url }
                            class=if link.url == current_url { "nav-link active" } else { "nav-link" }
                        >
                            <super::Svg icon=link.icon size=16 />
                            { link.label }
                            {
                                if link.count > 0 {
                                    yew::html! {
                                        <span class="badge badge-primary">{ link.count }</span>
                                    }
                                } else {
                                    "".into()
                                }
                            }
                        </a>
                    </li>
                })
            }
            </ul>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
