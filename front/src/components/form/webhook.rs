pub(crate) enum Message {
    Cancel,
    Submit,
    UpdateMarkRead(bool),
    UpdateName(String),
    UpdateUrl(String),
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub webhook: oxfeed_common::webhook::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed_common::webhook::Entity>,
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
            Message::Cancel => self.props.on_cancel.emit(()),
            Message::Submit => self.props.on_submit.emit(self.props.webhook.clone()),
            Message::UpdateMarkRead(mark_read) => self.props.webhook.mark_read = mark_read,
            Message::UpdateName(name) => self.props.webhook.name = name,
            Message::UpdateUrl(url) => self.props.webhook.url = url,
        }

        true
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <form>
                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="title">{ "Name" }</label>
                    <div class="col-sm-11">
                        <input
                            class="form-control"
                            name="name"
                            required=true
                            value=self.props.webhook.name.clone()
                            oninput=self.link.callback(|e: yew::InputData| Message::UpdateName(e.value))
                        />
                    </div>
                </div>

                <div class="row mb-3">
                    <label class="col-sm-1 col-form-label" for="url">{ "URL" }</label>
                    <div class="col-sm-11">
                        <input
                            class="form-control"
                            name="url"
                            required=true
                            value=self.props.webhook.url.clone()
                            oninput=self.link.callback(|e: yew::InputData| Message::UpdateUrl(e.value))
                        />
                    </div>
                </div>

                <div class="row mb-3">
                    <div class="col-sm-11 offset-sm-1">
                        <crate::components::Switch
                            id="mark_read"
                            label="Mark item as read after webhook call"
                            active=self.props.webhook.mark_read
                            on_toggle=self.link.callback(Message::UpdateMarkRead)
                        />
                    </div>
                </div>

                <a
                    class=yew::classes!("btn", "btn-primary")
                    title="Save"
                    onclick=self.link.callback(|_| Message::Submit)
                >
                    <crate::components::Svg icon="check" size=24 />
                    { "Save" }
                </a>

                <a
                    class=yew::classes!("btn", "btn-secondary")
                    title="Cancel"
                    onclick=self.link.callback(|_| Message::Cancel)
                >
                    <crate::components::Svg icon="x" size=24 />
                    { "Cancel" }
                </a>
            </form>
        }
    }

    crate::change!();
}
