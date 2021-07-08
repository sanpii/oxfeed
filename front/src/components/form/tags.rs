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
    values: Vec<String>,
    on_change: yew::Callback<Vec<String>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            values: props.values,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Add(value) => {
                if !self.values.contains(&value) {
                    self.values.push(value);
                }
            }
            Self::Message::Delete => {
                self.values.pop();
            }
            Self::Message::Remove(idx) => {
                self.values.remove(idx);
            }
        }

        let mut tags = self.values.clone();
        tags.sort();

        self.on_change.emit(tags);

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div class="form-control tags-input">
                {
                    for self.values.iter().enumerate().map(|(idx, tag)| {
                        yew::html! {
                            <crate::components::Tag
                                value=tag.clone()
                                editable=true
                                on_click=self.link.callback(move |_| Self::Message::Remove(idx))
                            />
                        }
                    })
                }
                <super::Autocomplete
                    what="tags"
                    on_select=self.link.callback(Self::Message::Add)
                    on_delete=self.link.callback(|_| Self::Message::Delete)
                />
            </div>
        }
    }

    crate::change!(values);
}
