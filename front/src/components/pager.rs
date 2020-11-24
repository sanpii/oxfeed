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

        let (start, end) = if self.0.page <= 9 {
            (1, 10.min(self.0.last_page))
        } else if self.0.page >= self.0.last_page - 9 {
            (self.0.last_page - 10, self.0.last_page)
        } else {
            (self.0.page - 4, self.0.page + 4)
        };

        yew::html! {
            <ul class="pagination justify-content-center">
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
                if start > 1 {
                    yew::html! {
                        <>
                            <li class="page-item">
                                <a class="page-link" href=format!("?page=1&limit={}", self.0.max_per_page)>{ "1" }</a>
                            </li>
                            <li class="page-item disabled">
                                <a class="page-link" href="#">{ "…" }</a>
                            </li>
                        </>
                    }
                } else {
                    "".into()
                }
            }
            {
                for (start..end + 1).map(|i| if i == self.0.page {
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
                if end < self.0.last_page {
                    yew::html! {
                        <>
                            <li class="page-item disabled">
                                <a class="page-link" href="#">{ "…" }</a>
                            </li>
                            <li class="page-item">
                                <a class="page-link" href=format!("?page={}&limit={}", self.0.last_page, self.0.max_per_page)>{ self.0.last_page }</a>
                            </li>
                        </>
                    }
                } else {
                    "".into()
                }
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
