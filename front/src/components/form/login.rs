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
            Self::Message::Submit => {
                let info = Info {
                    email: self.email.clone(),
                    password: self.password.clone(),
                    remember_me: self.remember_me,
                };

                self.on_submit.emit(info);
            }
            Self::Message::ToggleRemember => self.remember_me = !self.remember_me,
            Self::Message::UpdateEmail(email) => self.email = email,
            Self::Message::UpdatePassword(password) => self.password = password,
        }

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <>
                <label for="email" class="sr-only">{ "Email" }</label>
                <input
                    type="email"
                    name="email"
                    class="form-control"
                    placeholder="Email"
                    required=true
                    autofocus=true
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateEmail(e.value))
                />
                <label for="password" class="sr-only">{ "Password" }</label>
                <input
                    type="password"
                    name="password"
                    class="form-control"
                    placeholder="Password"
                    required=true
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdatePassword(e.value))
                />
                <div class="checkbox">
                    <label>
                        <input
                            type="checkbox"
                            checked=self.remember_me
                            onclick=self.link.callback(|_| Self::Message::ToggleRemember)
                        />{ " Remember me" }
                    </label>
                </div>
                <a
                    class=yew::classes!("btn", "btn-lg", "btn-primary", "btn-block")
                    type="submit"
                    onclick=self.link.callback(|_| Self::Message::Submit)
                >{ "Sign in" }</a>
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.on_submit = props.on_submit;

        false
    }
}
