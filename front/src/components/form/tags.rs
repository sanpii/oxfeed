#[derive(Clone)]
pub enum Message {
    Add(String),
    Delete,
    Remove(usize),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub values: Vec<String>,
    pub on_change: yew::Callback<Vec<String>>,
}

pub struct Component {
    values: Vec<String>,
    on_change: yew::Callback<Vec<String>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props().clone();

        Self {
            values: props.values,
            on_change: props.on_change,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Add(value) => {
                if !self.values.contains(&value) {
                    self.values.push(value);
                }
            }
            Message::Delete => {
                self.values.pop();
            }
            Message::Remove(idx) => {
                self.values.remove(idx);
            }
        }

        let mut tags = self.values.clone();
        tags.sort();

        self.on_change.emit(tags);

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div class="form-control tags-input">
                {
                    for self.values.iter().enumerate().map(|(idx, tag)| {
                        yew::html! {
                            <crate::components::Tag
                                value={ tag.clone() }
                                editable=true
                                on_click={ ctx.link().callback(move |_| Message::Remove(idx)) }
                            />
                        }
                    })
                }
                <super::Autocomplete
                    on_select={ ctx.link().callback(Message::Add) }
                    on_delete={ ctx.link().callback(|_| Message::Delete) }
                />
            </div>
        }
    }

    crate::change!(values);
}
