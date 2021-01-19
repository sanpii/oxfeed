pub(crate) enum Message {
    Cancel,
    Submit,
    ToggleActive,
    UpdateTags(Vec<String>),
    UpdateTitle(String),
    UpdateUrl(String),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub source: oxfeed_common::source::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed_common::source::Entity>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Cancel => self.props.on_cancel.emit(()),
            Self::Message::Submit => self.props.on_submit.emit(self.props.source.clone()),
            Self::Message::ToggleActive => self.props.source.active = !self.props.source.active,
            Self::Message::UpdateTags(tags) => self.props.source.tags = tags,
            Self::Message::UpdateTitle(title) => self.props.source.title = title,
            Self::Message::UpdateUrl(url) => self.props.source.url = url,
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <form>
                <div class="from-group">
                    <label for="title">{ "Title" }</label>
                    <input
                        class="form-control"
                        name="title"
                        value={ &self.props.source.title }
                        oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateTitle(e.value))
                    />
                    <small class="form-text text-muted">{ "Leave empty to use the feed title." }</small>
                </div>

                <div class="from-group">
                    <label for="url">{ "Feed URL" }</label>
                    <input
                        class="form-control"
                        name="url"
                        required=true
                        value={ &self.props.source.url }
                        oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateUrl(e.value))
                    />
                </div>

                <div class="from-group">
                    <label for="tags">{ "Tags" }</label>
                    <super::Tags
                        values=self.props.source.tags.clone()
                        on_change=self.link.callback(|tags| Self::Message::UpdateTags(tags))
                    />
                </div>

                <div class="from-group">
                    <div class=("custom-control", "custom-switch")>
                        <input
                            id="active"
                            type="checkbox"
                            class="custom-control-input"
                            checked=self.props.source.active
                            onclick=self.link.callback(|_| Self::Message::ToggleActive)
                        />
                        <label class="custom-control-label" for="active">{ "Active" }</label>
                    </div>
                </div>

                <a
                    class=("btn", "btn-primary")
                    title="Save"
                    onclick=self.link.callback(|_| Self::Message::Submit)
                >
                    <crate::components::Svg icon="check" size=24 />
                    { "Save" }
                </a>

                <a
                    class=("btn", "btn-danger")
                    title="Cancel"
                    onclick=self.link.callback(|_| Self::Message::Cancel)
                >
                    <crate::components::Svg icon="x" size=24 />
                    { "Cancel" }
                </a>
            </form>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
