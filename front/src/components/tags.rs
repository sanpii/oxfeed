pub(crate) enum Message {
    Error(oxfeed_common::Error),
    Update(Vec<oxfeed_common::Tag>),
    NeedUpdate,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub pagination: oxfeed_common::Pagination,
}

pub(crate) struct Component {
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    tags: Vec<oxfeed_common::Tag>,
    pagination: oxfeed_common::Pagination,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        let component = Self {
            event_bus: crate::event::Bus::dispatcher(),
            link,
            tags: Vec::new(),
            pagination: props.pagination,
        };

        component.link.send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        use yewtil::future::LinkFuture;

        match msg {
            Self::Message::Error(err) => self.event_bus.send(err.into()),
            Self::Message::NeedUpdate => {
                let pagination = self.pagination;

                self.link.send_future(async move {
                    crate::Api::tags_all(&pagination)
                        .await
                        .map_or_else(Self::Message::Error, Self::Message::Update)
                });
            }
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
