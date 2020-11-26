#[derive(Clone)]
pub(crate) enum Message {
    Choose(usize),
    Input(String),
    Key(String),
    Terms(Vec<String>),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::SearchTags(tags) => Self::Terms(tags.iterator),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub what: String,
    #[prop_or_default]
    pub on_input: yew::Callback<String>,
    #[prop_or_default]
    pub on_keydown: yew::Callback<String>,
}

pub(crate) struct Component {
    active: Option<usize>,
    api: crate::Api<Self>,
    link: yew::ComponentLink<Self>,
    terms: Vec<String>,
    what: String,
    on_input: yew::Callback<String>,
    on_keydown: yew::Callback<String>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            active: None,
            api: crate::Api::new(link.clone()),
            link,
            terms: Vec::new(),
            what: props.what,
            on_input: props.on_input,
            on_keydown: props.on_keydown,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Choose(idx) => {
                self.on_input.emit(self.terms[idx].clone());
                self.on_keydown.emit("Enter".to_string());
            },
            Self::Message::Input(input) => if !input.is_empty() {
                self.api.search(&self.what, &input, &crate::Pagination::new());
                self.on_input.emit(input.clone());
                return false;
            } else {
                self.terms = Vec::new();
                self.active = None;
            },
            Self::Message::Key(key) => match key.as_str() {
                "ArrowDown" => self.active = if let Some(active) = self.active {
                    Some((active + 1) % self.terms.len())
                } else {
                    Some(0)
                },
                "ArrowUp" => self.active = if let Some(active) = self.active {
                    Some(active.checked_sub(1).unwrap_or(self.terms.len() - 1))
                } else {
                    Some(self.terms.len() - 1)
                },
                "Escape" => {
                    self.terms = Vec::new();
                    self.active = None;
                },
                key => self.on_keydown.emit(key.to_string()),
            },
            Self::Message::Terms(terms) => self.terms = terms,
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <>
                <input
                    type="text"
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::Input(e.value))
                    onkeydown=self.link.callback(|e: yew::KeyboardEvent| Self::Message::Key(e.key()))
                />
                {
                    if !self.terms.is_empty() {
                        yew::html! {
                            <div class=("list-group", "autocomplete")>
                            {
                                for self.terms.iter().enumerate().map(|(idx, term)| {
                                    yew::html! {
                                        <div
                                            class=("list-group-item", "list-group-item-action", if self.active == Some(idx) { "active" } else { "" })
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
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
