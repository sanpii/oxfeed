#[derive(Clone, yew::Properties)]
pub(crate) struct Properties<R: crate::Render> {
    pub value: crate::Pager<R>,
}

pub(crate) struct Component<R: 'static + crate::Render> {
    pager: crate::Pager<R>,
}

impl<R: 'static + crate::Render> yew::Component for Component<R> {
    type Properties = Properties<R>;
    type Message = ();

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self { pager: props.value }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        if self.pager.iterator.is_empty() {
            return yew::html! {
                <super::Empty />
            }
        }

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
                <super::Pager<R> value=self.pager.clone() />
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.pager != props.value;

        self.pager = props.value;

        should_render
    }
}
