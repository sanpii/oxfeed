pub enum Message {
    Page(usize),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties<R: crate::Render> {
    pub value: crate::Pager<R>,
    #[prop_or_default]
    pub on_page_change: yew::Callback<usize>,
}

pub struct Component<R: 'static + crate::Render> {
    value: crate::Pager<R>,
    on_page_change: yew::Callback<usize>,
}

impl<R: 'static + crate::Render> yew::Component for Component<R> {
    type Message = Message;
    type Properties = Properties<R>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props().clone();

        Self {
            value: props.value,
            on_page_change: props.on_page_change,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        let Message::Page(page) = msg;
        self.on_page_change.emit(page);

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        if self.value.iterator.is_empty() {
            return yew::html! {
                <super::Empty />
            };
        }

        let value: elephantry_extras::Pager = self.value.clone().into();

        yew::html! {
            <>
                <ul class="list-group">
                {
                    for self.value.iterator.iter().map(|item| {
                        yew::html! {
                            <li class="list-group-item">{ item.render() }</li>
                        }
                    })
                }
                </ul>
                <elephantry_extras::yew::Pager
                    value={ value }
                    onclick={ ctx.link().callback(Message::Page) }
                />
            </>
        }
    }

    crate::change!(value, on_page_change);
}
