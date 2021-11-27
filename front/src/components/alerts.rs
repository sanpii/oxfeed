pub(crate) enum Message {
    Close(usize),
    Event(crate::Event),
}

pub(crate) struct Component {
    messages: Vec<crate::event::Alert>,
    _producer: Box<dyn yew_agent::Bridge<crate::event::Bus>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        use yew_agent::Bridged;

        let callback = ctx.link().callback(Message::Event);

        Self {
            messages: Vec::new(),
            _producer: crate::event::Bus::bridge(callback),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Event(event) => {
                if let crate::Event::Alert(alert) = event {
                    self.messages.push(alert);
                    should_render = true;
                }
            }
            Message::Close(idx) => {
                self.messages.remove(idx);
                should_render = true;
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            {
                for self.messages.iter().enumerate().map(|(idx, alert)| {
                    yew::html! {
                        <div class={ yew::classes!("alert", format!("alert-{}", alert.severity()), "alert-dismissible") } role="alert">
                            { &alert.message }

                            <button
                                class="btn-close"
                                onclick={ ctx.link().callback(move |_| Message::Close(idx)) }
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
