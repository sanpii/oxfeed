pub enum Message {
    UpdateEmail(String),
    UpdatePassword(String),
    Submit,
}

pub struct Info {
    pub email: String,
    pub password: String,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub on_submit: yew::Callback<Info>,
}

pub struct Component {
    email: String,
    password: String,
    on_submit: yew::Callback<Info>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            on_submit: ctx.props().on_submit.clone(),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Submit => {
                let info = Info {
                    email: self.email.clone(),
                    password: self.password.clone(),
                };
                self.on_submit.emit(info);
            }
            Message::UpdateEmail(email) => self.email = email,
            Message::UpdatePassword(password) => self.password = password,
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use yew::TargetCast;

        yew::html! {
            <>
                <div class="form-floating">
                    <input
                        type="email"
                        name="email"
                        class="form-control"
                        placeholder="Email"
                        required=true
                        oninput={ ctx.link().callback(|e: yew::InputEvent| {
                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            Message::UpdateEmail(input.value())
                        }) }
                    />
                    <label for="email" class="form-label sr-only">{ "Email" }</label>
                </div>
                <div class="form-floating">
                    <label for="password" class="form-label sr-only">{ "Password" }</label>
                    <input
                        type="password"
                        name="password"
                        class="form-control"
                        placeholder="Password"
                        required=true
                        oninput={ ctx.link().callback(|e: yew::InputEvent| {
                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            Message::UpdatePassword(input.value())
                        }) }
                    />
                </div>
                <a
                    class={ yew::classes!("btn", "btn-lg", "btn-primary", "w-100") }
                    type="submit"
                    onclick={ ctx.link().callback(|_| Message::Submit) }
                >{ "Register" }</a>
            </>
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, _: &Self::Properties) -> bool {
        self.on_submit = ctx.props().on_submit.clone();

        false
    }
}
