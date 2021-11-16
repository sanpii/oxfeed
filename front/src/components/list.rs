pub enum Message {
    Page(usize),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties<R: crate::Render> {
    pub value: crate::Pager<R>,
    #[prop_or_default]
    pub on_page_change: yew::Callback<usize>,
}

pub(crate) struct Component<R: 'static + crate::Render> {
    link: yew::ComponentLink<Self>,
    value: crate::Pager<R>,
    on_page_change: yew::Callback<usize>,
}

impl<R: 'static + crate::Render> yew::Component for Component<R> {
    type Message = Message;
    type Properties = Properties<R>;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            value: props.value,
            on_page_change: props.on_page_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let Message::Page(page) = msg;
        self.on_page_change.emit(page);

        false
    }

    fn view(&self) -> yew::Html {
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
                    value=value
                    onclick=self.link.callback(Message::Page)
                />
            </>
        }
    }

    crate::change!(value, on_page_change);
}
