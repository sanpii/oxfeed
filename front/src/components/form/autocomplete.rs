pub enum Message {
    Choose(usize),
    Error(String),
    Input(String),
    Key(String),
    Terms(crate::Pager<String>),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    #[prop_or_default]
    pub on_select: yew::Callback<String>,
    #[prop_or_default]
    pub on_delete: yew::Callback<()>,
}

pub struct Component {
    active: Option<usize>,
    input_ref: yew::NodeRef,
    terms: Vec<String>,
    value: String,
    on_select: yew::Callback<String>,
    on_delete: yew::Callback<()>,
}

impl Component {
    fn select(&mut self, value: String) {
        self.on_select.emit(value);
        self.active = None;
        self.terms = Vec::new();
        self.value = String::new();
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props().clone();

        Self {
            active: None,
            input_ref: yew::NodeRef::default(),
            terms: Vec::new(),
            value: String::new(),
            on_select: props.on_select,
            on_delete: props.on_delete,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = true;

        match msg {
            Message::Choose(idx) => self.select(self.terms[idx].clone()),
            Message::Error(err) => crate::send_error(ctx, &err),
            Message::Input(input) => {
                self.value.clone_from(&input);

                let pagination = elephantry_extras::Pagination::new();
                let filter: crate::Filter = input.clone().into();

                if input.is_empty() {
                    self.terms = Vec::new();
                    self.active = None;
                } else {
                    crate::api!(
                        ctx.link(),
                        tags_search(filter, pagination) -> Message::Terms
                    );

                    should_render = false;
                }
            }
            Message::Key(key) => match key.as_str() {
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
            Message::Terms(pager) => self.terms = pager.iterator,
        }

        should_render
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {
        if !self.terms.is_empty() {
            return;
        }

        if let Some(input) = self.input_ref.cast::<web_sys::HtmlInputElement>() {
            input.focus().unwrap();
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class="autocomplete">
                <input
                    type="text"
                    ref={ self.input_ref.clone() }
                    value={ self.value.clone() }
                    oninput={ ctx.link().callback(|e: yew::InputEvent| {
                        use yew::TargetCast;

                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        Message::Input(input.value())
                    }) }
                    onkeydown={ ctx.link().callback(|e: yew::KeyboardEvent| Message::Key(e.key())) }
                />
                {
                    if self.terms.is_empty() {
                        "".into()
                    } else {
                        yew::html! {
                            <div class="list-group">
                            {
                                for self.terms.iter().enumerate().map(|(idx, term)| {
                                    yew::html! {
                                        <div
                                            class={ yew::classes!("list-group-item", "list-group-item-action", if self.active == Some(idx) { "active" } else { "" }) }
                                            onclick={ ctx.link().callback(move |_| Message::Choose(idx)) }
                                        >{ term }</div>
                                    }
                                })
                            }
                            </div>
                        }
                    }
                }
            </div>
        }
    }

    crate::change!();
}
