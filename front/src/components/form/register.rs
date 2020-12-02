pub(crate) enum Message {
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateName(String),
    Submit,
}

pub(crate) struct Info {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Clone, yew::Properties)]
pub(crate) struct Properties {
    pub on_submit: yew::Callback<Info>,
}

pub(crate) struct Component {
    link: yew::ComponentLink<Self>,
    email: String,
    password: String,
    name: String,
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
            name: String::new(),
            on_submit: props.on_submit,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Submit => {
                let info = Info {
                    email: self.email.clone(),
                    password: self.password.clone(),
                    name: self.name.clone(),
                };
                self.on_submit.emit(info);
            },
            Self::Message::UpdateEmail(email) => self.email = email,
            Self::Message::UpdatePassword(password) => self.password = password,
            Self::Message::UpdateName(name) => self.name = name,
        }

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <>
                <label for="name" class="sr-only">{ "Name" }</label>
                <input
                    type="text"
                    name="name"
                    class="form-control"
                    placeholder="Name"
                    required=true
                    autofocus=true
                    oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateName(e.value))
                />
                <label for="email" class="sr-only">{ "Email" }</label>
                <input
                    type="email"
                    name="email"
                    class="form-control"
                    placeholder="Email"
                    required=true
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
                <a
                    class=("btn", "btn-lg", "btn-primary", "btn-block")
                    type="submit"
                    onclick=self.link.callback(|_| Self::Message::Submit)
                >{ "Register" }</a>
            </>
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.on_submit = props.on_submit;

        false
    }
}
