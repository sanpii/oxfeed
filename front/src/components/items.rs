#[derive(Clone)]
pub(crate) enum Message {
    Error(String),
    Event(crate::event::Event),
    NeedUpdate,
    Update(crate::Pager<crate::Item>),
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = ();

    fn try_from((_, response): (http::Method, yew::format::Text)) -> Result<Self, ()> {
        let data = match response {
            Ok(data) => data,
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        let message = match serde_json::from_str(&data) {
            Ok(sources) => Self::Update(sources),
            Err(err) => return Ok(Self::Error(err.to_string())),
        };

        Ok(message)
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: String,
    pub pagination: crate::Pagination,
}

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    filter: String,
    pager: Option<crate::Pager<crate::Item>>,
    pagination: crate::Pagination,
    link: yew::ComponentLink<Self>,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl Component {
    fn url(filter: &str, pagination: &crate::Pagination) -> String {
        let mut url = filter.to_string();

        if !url.contains('?') {
            url.push('?');
        } else {
            url.push('&');
        }

        format!("{}page={}&limit={}", url, pagination.page, pagination.limit)
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Bridged;

        let callback = link.callback(|x| Self::Message::Event(x));

        let component = Self {
            fetch_task: None,
            filter: props.filter,
            pager: None,
            pagination: props.pagination,
            link,
            _producer: crate::event::Bus::bridge(callback),
        };

        component.link.send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Error(error) => {
                log::error!("{:?}", error);
                return false;
            },
            Self::Message::Event(event) =>  match event {
                crate::event::Event::ItemUpdate => self.link.send_message(Self::Message::NeedUpdate),
                _ => (),
            },
            Self::Message::NeedUpdate => {
                let url = Self::url(&self.filter, &self.pagination);
                self.fetch_task = crate::get(&self.link, &url, yew::format::Nothing).ok();
                return false;
            },
            Self::Message::Update(ref pager) => self.pager = Some(pager.clone()),
        }

        true
    }

    fn view(&self) -> yew::Html {
        let pager = match &self.pager {
            Some(pager) => pager,
            None => return "Nothing found".into(),
        };

        if pager.iterator.is_empty() {
            return "Nothing found".into();
        }

        yew::html! {
            <>
                <ul class="list-group">
                {
                    for pager.iterator.iter().map(|item| {
                        yew::html! {
                            <li class="list-group-item">
                                <super::Item value=item />
                            </li>
                        }
                    })
                }
                </ul>
                <super::Pager<crate::Item> base_url=self.filter.clone() value=pager />
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.filter != props.filter || self.pagination != props.pagination;

        self.filter = props.filter;
        self.pagination = props.pagination;
        self.link.send_message(Self::Message::NeedUpdate);

        should_render
    }
}
