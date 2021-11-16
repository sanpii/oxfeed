pub(crate) enum Message {
    Choose(usize),
    Error(oxfeed_common::Error),
    Input(String),
    Key(String),
    Terms(Vec<String>),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub on_select: yew::Callback<String>,
    #[prop_or_default]
    pub on_delete: yew::Callback<()>,
}

pub(crate) struct Component {
    active: Option<usize>,
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    terms: Vec<String>,
    value: String,
    on_select: yew::Callback<String>,
    on_delete: yew::Callback<()>,
}

impl Component {
    fn select(&mut self, value: String) {
        self.on_select.emit(value);
        self.active = None;
        self.value = String::new();
        self.terms = Vec::new();
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::Dispatched;

        Self {
            active: None,
            event_bus: crate::event::Bus::dispatcher(),
            link,
            terms: Vec::new(),
            value: String::new(),
            on_select: props.on_select,
            on_delete: props.on_delete,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Choose(idx) => self.select(self.terms[idx].clone()),
            Self::Message::Error(error) => self.event_bus.send(error.into()),
            Self::Message::Input(input) => {
                self.value = input.clone();

                let pagination = oxfeed_common::Pagination::new();
                let filter: crate::Filter = input.clone().into();

                if !input.is_empty() {
                    crate::api!(
                        self.link,
                        tags_search(filter, pagination) -> Message::Terms, Message::Error
                    );

                    return false;
                } else {
                    self.terms = Vec::new();
                    self.active = None;
                }
            }
            Self::Message::Key(key) => match key.as_str() {
                "ArrowDown" => {
                    self.active = if let Some(active) = self.active {
                        Some((active + 1) % self.terms.len())
                    } else {
                        Some(0)
                    }
                }
                "ArrowUp" => {
                    self.active = if let Some(active) = self.active {
                        Some(active.checked_sub(1).unwrap_or(self.terms.len() - 1))
                    } else {
                        Some(self.terms.len() - 1)
                    }
                }
                "Backspace" => {
                    if self.value.is_empty() {
                        self.on_delete.emit(());
                    }
                }
                "Enter" => {
                    if let Some(active) = self.active {
                        self.select(self.terms[active].clone());
                    } else {
                        self.select(self.value.clone());
                    }
                }
                "Escape" => {
                    self.terms = Vec::new();
                    self.active = None;
                }
                _ => (),
            },
            Self::Message::Terms(terms) => self.terms = terms,
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div class="autocomplete">
                <input
                    type="text"
                    value=self.value.clone()
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::Input(e.value))
                    onkeydown=self.link.callback(|e: yew::KeyboardEvent| Self::Message::Key(e.key()))
                />
                {
                    if !self.terms.is_empty() {
                        yew::html! {
                            <div class="list-group">
                            {
                                for self.terms.iter().enumerate().map(|(idx, term)| {
                                    yew::html! {
                                        <div
                                            class=yew::classes!("list-group-item", "list-group-item-action", if self.active == Some(idx) { "active" } else { "" })
                                            onclick=self.link.callback(move |_| Self::Message::Choose(idx))
                                        >{ term }</div>
                                    }
                                })
                            }
                            </div>
                        }
                    } else {
                        "".into()
                    }
                }
            </div>
        }
    }

    crate::change!();
}
