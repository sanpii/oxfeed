#[derive(Clone)]
pub(crate) enum Message {
    Update(Vec<oxfeed_common::Tag>),
    NeedUpdate,
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Tags(tags) => Self::Update(tags),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    link: yew::ComponentLink<Self>,
    tags: Vec<oxfeed_common::Tag>,
    pagination: oxfeed_common::Pagination,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let component = Self {
            api: crate::Api::new(link.clone()),
            link,
            tags: Vec::new(),
            pagination: props.pagination,
        };

        component.link.send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::NeedUpdate => self.api.tags_all(&self.pagination),
            Self::Message::Update(tags) => {
                self.tags = tags;
                return true;
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        if self.tags.is_empty() {
            return yew::html! {
                <super::Empty />
            };
        }

        let max = self.tags.iter().map(|x| x.count).max().unwrap_or(1);

        yew::html! {
            <div class=yew::classes!("d-flex", "flex-wrap", "align-items-center", "cloud")>
            {
                for self.tags.iter().map(|tag| {
                    let style = format!("font-size: {}rem", tag.count as f32 / max as f32 * 5. + 1.);
                    let href = format!("/search/all?tag={}", tag.name);

                    yew::html! {
                        <div style=style>
                            <a href=href>
                                <crate::components::Tag value=tag.name.clone() />
                            </a>
                        </div>
                    }
                })
            }
            </div>
        }
    }

    crate::change!();
}
