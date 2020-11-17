pub(crate) enum Message {
    Cancel,
    Submit,
    UpdateUrl(String),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub source: crate::Source,
    pub oncancel: yew::Callback<()>,
    pub onsubmit: yew::Callback<crate::Source>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Cancel => self.props.oncancel.emit(()),
            Self::Message::Submit => self.props.onsubmit.emit(self.props.source.clone()),
            Self::Message::UpdateUrl(url) => self.props.source.url = url,
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <form>
                <div class="from-group">
                    <label for="url">{ "Feed URL" }</label>
                    <input
                        class="form-control"
                        name="url"
                        required=true
                        value={ &self.props.source.url }
                        oninput=self.link.callback(|e: yew::InputData| Message::UpdateUrl(e.value))
                    />
                </div>

                <a
                    class=("btn", "btn-primary")
                    title="Save"
                    onclick=self.link.callback(|_| Message::Submit)
                >
                    <super::Svg icon="check" size=24 />
                    { "Save" }
                </a>

                <a
                    class=("btn", "btn-danger")
                    title="Cancel"
                    onclick=self.link.callback(|_| Message::Cancel)
                >
                    <super::Svg icon="x" size=24 />
                    { "Cancel" }
                </a>
            </form>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
