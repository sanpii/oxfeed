#[derive(Clone, Eq, PartialEq, yew::Properties)]
pub(crate) struct Properties<T: Clone + Eq + PartialEq + serde::de::DeserializeOwned> {
    pub value: crate::Pager<T>,
}

pub(crate) struct Component<T: Clone + Eq + PartialEq + serde::de::DeserializeOwned>(crate::Pager<T>);

impl<T: 'static + Clone + Eq + serde::de::DeserializeOwned> yew::Component for Component<T> {
    type Message = ();
    type Properties = Properties<T>;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self(props.value)
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        if self.0.last_page <= 1 {
            return "".into();
        }

        yew::html! {
            <ul class="pagination">
            {
                if self.0.has_previous_page {
                    yew::html! {
                        <li class="page-item">
                            <a class="page-link" href=format!("?page={}&limit={}", self.0.page - 1, self.0.max_per_page)>{ "«" }</a>
                        </li>
                    }
                } else {
                    yew::html! {
                        <li class="page-item disabled">
                            <a class="page-link" href="#">{ "«" }</a>
                        </li>
                    }
                }
            }
            {
                for (1..self.0.last_page + 1).map(|i| if i == self.0.page {
                        yew::html! {
                            <li class="page-item active"><a class="page-link" href="#">{ self.0.page } <span class="sr-only">{ "(current)" }</span></a></li>
                        }
                    } else {
                        yew::html! {
                            <li class="page-item"><a class="page-link" href=format!("?page={}&limit={}", i, self.0.max_per_page)>{ i }</a></li>
                        }
                    })
            }
            {
                if self.0.has_next_page {
                    yew::html! {
                        <li class="page-item">
                            <a class="page-link" href=format!("?page={}&limit={}", self.0.page + 1, self.0.max_per_page)>{ "»" }</a>
                        </li>
                    }
                } else {
                    yew::html! {
                        <li class="page-item disabled">
                            <a class="page-link" href="#">{ "»" }</a>
                        </li>
                    }
                }
            }
            </ul>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.0 != props.value;

        self.0 = props.value;

        should_render
    }
}
