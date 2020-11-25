#[derive(Clone, Eq, PartialEq, yew::Properties)]
pub(crate) struct Properties<T: Clone + Eq + PartialEq + serde::de::DeserializeOwned> {
    #[prop_or_default]
    pub base_url: String,
    pub value: crate::Pager<T>,
}

pub(crate) struct Component<T: Clone + Eq + PartialEq + serde::de::DeserializeOwned> {
    base_url: String,
    pager: crate::Pager<T>,
}

impl<T: 'static + Clone + Eq + serde::de::DeserializeOwned> Component<T> {
    fn url(&self, page: usize, max_per_page: usize) -> String {
        let mut url = self.base_url.clone();

        if url.is_empty() {
            url = "?".to_string();
        } else {
            if !url.contains('?') {
                url.push('?');
            } else {
                url.push('&');
            }
        }

        format!("{}page={}&limit={}", url, page, max_per_page)
    }
}

impl<T: 'static + Clone + Eq + serde::de::DeserializeOwned> yew::Component for Component<T> {
    type Message = ();
    type Properties = Properties<T>;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self {
            base_url: props.base_url,
            pager: props.value,
        }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        if self.pager.last_page <= 1 {
            return "".into();
        }

        let (start, end) = if self.pager.page <= 9 {
            (1, 10.min(self.pager.last_page))
        } else if self.pager.page >= self.pager.last_page - 9 {
            (self.pager.last_page - 10, self.pager.last_page)
        } else {
            (self.pager.page - 4, self.pager.page + 4)
        };

        yew::html! {
            <ul class="pagination justify-content-center">
            {
                if self.pager.has_previous_page {
                    yew::html! {
                        <li class="page-item">
                            <a class="page-link" href=self.url(self.pager.page - 1, self.pager.max_per_page)>{ "«" }</a>
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
                                <a class="page-link" href=self.url(1, self.pager.max_per_page)>{ "1" }</a>
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
                for (start..end + 1).map(|i| if i == self.pager.page {
                        yew::html! {
                            <li class="page-item active"><a class="page-link" href="#">{ self.pager.page } <span class="sr-only">{ "(current)" }</span></a></li>
                        }
                    } else {
                        yew::html! {
                            <li class="page-item"><a class="page-link" href=self.url(i, self.pager.max_per_page)>{ i }</a></li>
                        }
                    })
            }
            {
                if end < self.pager.last_page {
                    yew::html! {
                        <>
                            <li class="page-item disabled">
                                <a class="page-link" href="#">{ "…" }</a>
                            </li>
                            <li class="page-item">
                                <a class="page-link" href=self.url(self.pager.last_page, self.pager.max_per_page)>{ self.pager.last_page }</a>
                            </li>
                        </>
                    }
                } else {
                    "".into()
                }
            }
            {
                if self.pager.has_next_page {
                    yew::html! {
                        <li class="page-item">
                            <a class="page-link" href=self.url(self.pager.page + 1, self.pager.max_per_page)>{ "»" }</a>
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
        let should_render = self.base_url != props.base_url || self.pager != props.value;

        self.base_url = props.base_url;
        self.pager = props.value;

        should_render
    }
}
