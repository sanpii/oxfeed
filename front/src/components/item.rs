#[derive(Clone)]
pub(crate) enum Message {
    Nothing,
}

impl std::convert::TryFrom<yew::format::Text> for Message {
    type Error = ();

    fn try_from(_: yew::format::Text) -> Result<Self, ()> {
        Ok(Message::Nothing)
    }
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub value: crate::Item,
}

pub(crate) struct Component {
    item: crate::Item,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self {
            item: props.value,
        }
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let empty_img = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7".to_string();
        // @FIXME https://gitlab.com/imp/chrono-humanize-rs/-/merge_requests/5
        let published_ago = chrono_humanize::HumanTime::from(self.item.published - chrono::Utc::now());

        yew::html! {
            <>
                <img src=self.item.icon.as_ref().unwrap_or(&empty_img) width="16" height="16" />
                <a href=self.item.link.clone() target="_blank">{ &self.item.title }</a>
                <span class="text-muted">{ " · " }{ &self.item.source }</span>
                <span class=("text-muted", "float-right")>{ &published_ago }</span>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
