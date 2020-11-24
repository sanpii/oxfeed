#[derive(Clone)]
pub(crate) enum Message {
    Add,
    Delete,
    Nope,
    Remove(usize),
    Update(String),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub values: Vec<String>,
    pub on_change: yew::Callback<Vec<String>>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    tags: Vec<String>,
    value: String,
    on_change: yew::Callback<Vec<String>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            tags: props.values,
            value: String::new(),
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Add => if !self.value.is_empty() {
                if !self.tags.contains(&self.value) {
                    self.tags.push(self.value.clone());
                }
                self.value = String::new();
            },
            Self::Message::Delete => if self.value.is_empty() {
                self.tags.pop();
            },
            Self::Message::Remove(idx) => {
                self.tags.remove(idx);
            },
            Self::Message::Update(value) => {
                self.value = value;
                return false;
            },
            Self::Message::Nope => return false,
        }

        self.on_change.emit(self.tags.clone());

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div class="form-control tags-input">
                {
                    for self.tags.iter().enumerate().map(|(idx, tag)| {
                        yew::html! {
                            <crate::components::Tag
                                value=tag
                                on_click=self.link.callback(move |_| Self::Message::Remove(idx))
                            />
                        }
                    })
                }
                <input
                    type="text"
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::Update(e.value))
                    onkeydown=self.link.callback(|e: yew::KeyboardEvent| match e.key().as_str() {
                        "Enter" => Self::Message::Add,
                        "Backspace" => Self::Message::Delete,
                        _ => Self::Message::Nope,
                    })
                />
            </div>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let should_render = self.tags != props.values;

        self.tags = props.values;

        should_render
    }
}
