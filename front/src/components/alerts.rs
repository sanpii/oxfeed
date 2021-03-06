pub(crate) enum Message {
    Close(usize),
    Event(crate::event::Event),
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    messages: Vec<crate::event::Alert>,
    _producer: Box<dyn yew::agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::Bridged;

        let callback = link.callback(Self::Message::Event);

        Self {
            link,
            messages: Vec::new(),
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Event(event) => {
                if let crate::event::Event::Alert(alert) = event {
                    self.messages.push(alert);
                    return true;
                }
            }
            Self::Message::Close(idx) => {
                self.messages.remove(idx);
                return true;
            }
        };

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            {
                for self.messages.iter().enumerate().map(|(idx, alert)| {
                    yew::html! {
                        <div class=yew::classes!("alert", format!("alert-{}", alert.severity()), "alert-dismissible") role="alert">
                            { &alert.message }

                            <button
                                class="btn-close"
                                onclick=self.link.callback(move |_| Self::Message::Close(idx))
                            >
                            </button>
                        </div>
                    }
                })
            }
        }
    }

    crate::change!();
}
