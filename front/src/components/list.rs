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
    pager: crate::Pager<R>,
    on_page_change: yew::Callback<usize>,
}

impl<R: 'static + crate::Render> yew::Component for Component<R> {
    type Message = Message;
    type Properties = Properties<R>;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            pager: props.value,
            on_page_change: props.on_page_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let Message::Page(page) = msg;
        self.on_page_change.emit(page);

        false
    }

    fn view(&self) -> yew::Html {
        if self.pager.iterator.is_empty() {
            return yew::html! {
                <super::Empty />
            };
        }

        let pager: elephantry_extras::Pager = self.pager.clone().into();

        yew::html! {
            <>
                <ul class="list-group">
                {
                    for self.pager.iterator.iter().map(|item| {
                        yew::html! {
                            <li class="list-group-item">{ item.render() }</li>
                        }
                    })
                }
                </ul>
                <elephantry_extras::yew::Pager
                    value=pager
                    onclick=self.link.callback(|page| Self::Message::Page(page))
                />
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.pager != props.value;

        self.pager = props.value;
        self.on_page_change = props.on_page_change;

        should_render
    }
}
