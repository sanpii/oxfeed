pub(crate) enum Message {
    ToggleRemember,
    UpdateEmail(String),
    UpdatePassword(String),
    Submit,
}

pub(crate) struct Info {
    pub email: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub on_submit: yew::Callback<Info>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    email: String,
    password: String,
    remember_me: bool,
    on_submit: yew::Callback<Info>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            email: String::new(),
            password: String::new(),
            remember_me: false,
            on_submit: props.on_submit,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::Submit => {
                let info = Info {
                    email: self.email.clone(),
                    password: self.password.clone(),
                    remember_me: self.remember_me,
                };

                self.on_submit.emit(info);
            }
            Message::ToggleRemember => self.remember_me = !self.remember_me,
            Message::UpdateEmail(email) => self.email = email,
            Message::UpdatePassword(password) => self.password = password,
        }

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <>
                <div class="form-floating">
                    <input
                        type="email"
                        name="email"
                        class="form-control"
                        placeholder="Email"
                        required=true
                        autofocus=true
                        oninput=self.link.callback(|e: yew::InputData| Message::UpdateEmail(e.value))
                    />
                    <label for="email" class="form-label sr-only">{ "Email" }</label>
                </div>
                <div class="form-floating">
                    <input
                        type="password"
                        name="password"
                        class="form-control"
                        placeholder="Password"
                        required=true
                        oninput=self.link.callback(|e: yew::InputData| Message::UpdatePassword(e.value))
                    />
                    <label for="password" class="form-label sr-only">{ "Password" }</label>
                </div>
                <div class="checkbox">
                    <label class="form-label">
                        <input
                            type="checkbox"
                            checked=self.remember_me
                            onclick=self.link.callback(|_| Message::ToggleRemember)
                        />{ " Remember me" }
                    </label>
                </div>
                <a
                    class=yew::classes!("btn", "btn-lg", "btn-primary", "w-100")
                    type="submit"
                    onclick=self.link.callback(|_| Message::Submit)
                >{ "Sign in" }</a>
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.on_submit = props.on_submit;

        false
    }
}
