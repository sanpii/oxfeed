#[derive(Clone)]
pub(crate) enum Message {
    Add(String),
    Delete,
    Remove(usize),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub values: Vec<String>,
    pub on_change: yew::Callback<Vec<String>>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    tags: Vec<String>,
    on_change: yew::Callback<Vec<String>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            tags: props.values,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Add(value) => {
                if !self.tags.contains(&value) {
                    self.tags.push(value);
                }
            }
            Self::Message::Delete => {
                self.tags.pop();
            }
            Self::Message::Remove(idx) => {
                self.tags.remove(idx);
            }
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
                <super::Autocomplete
                    what="tags"
                    on_select=self.link.callback(|value| Self::Message::Add(value))
                    on_delete=self.link.callback(|_| Self::Message::Delete)
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
