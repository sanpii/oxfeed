pub(crate) enum Message {
    Delete,
    Update,
}

impl From<yew::format::Text> for Message {
    fn from(_: yew::format::Text) -> Self {
        Self::Update
    }
}

#[derive(yew::Properties, Clone)]
pub(crate) struct Properties {
    pub value: crate::Source,
}

pub(crate) struct Component {
    fetch_task: Option<yew::services::fetch::FetchTask>,
    link: yew::ComponentLink<Self>,
    source: crate::Source,
}

impl Component {
    fn delete(&mut self) {
        self.fetch_task = crate::delete(&self.link, &format!("/sources/{}", self.source.source_id)).ok();
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            link,
            source: props.value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Delete => if yew::services::dialog::DialogService::confirm(&format!("Would you like delete '{}' source?", self.source.title)) {
                self.delete();
                return true;
            },
            Self::Message::Update => {
                let parent = self.link.get_parent().unwrap();
                let sources = parent.clone().downcast::<super::Sources>();

                sources.send_message(super::sources::Message::NeedUpdate);
            },
        };

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <>
                { &self.source.title }

                <div class="btn-group float-right">
                    <button
                        class="btn btn-danger"
                        title="Delete"
                        onclick=self.link.callback(|_| Message::Delete)
                    >
                        <super::Svg icon="trash" size=24 />
                    </button>
                </div>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
