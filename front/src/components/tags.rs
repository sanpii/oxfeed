pub enum Message {
    Error(String),
    Update(Vec<oxfeed_common::Tag>),
    NeedUpdate,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub pagination: oxfeed_common::Pagination,
}

pub struct Component {
    tags: Vec<oxfeed_common::Tag>,
    pagination: oxfeed_common::Pagination,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = Message;

    fn create(context: &yew::Context<Self>) -> Self {
        let component = Self {
            tags: Vec::new(),
            pagination: context.props().pagination,
        };

        context.link().send_message(Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Error(err) => crate::send_error(ctx, &err),
            Message::NeedUpdate => {
                let pagination = &self.pagination;

                crate::api!(
                    ctx.link(),
                    tags_all(pagination) -> Message::Update
                );
            }
            Message::Update(tags) => {
                self.tags = tags;
                should_render = true;
            }
        }

        should_render
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        if self.tags.is_empty() {
            return yew::html! {
                <super::Empty />
            };
        }

        let max = self.tags.iter().map(|x| x.count).max().unwrap_or(1);

        yew::html! {
            <div class={ yew::classes!("d-flex", "flex-wrap", "align-items-center", "cloud") }>
            {
                for self.tags.iter().map(|tag| {
                    let style = format!("font-size: {}rem", tag.count as f32 / max as f32 * 5. + 1.);
                    let href = format!("/search/all?tag={}", tag.name);

                    yew::html! {
                        <div style={ style }>
                            <a href={ href }>
                                <crate::components::Tag value={ tag.name.clone() } />
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
