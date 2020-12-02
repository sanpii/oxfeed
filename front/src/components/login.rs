pub(crate) enum Message {
    Login,
    ToggleRemember,
    UpdateLogin(String),
    UpdatePassword(String),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Auth => Self::Login,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct Component {
    api: crate::Api<Self>,
    link: yew::ComponentLink<Self>,
    login: String,
    password: String,
    remember_me: bool,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            api: crate::Api::new(link.clone()),
            link,
            login: String::new(),
            password: String::new(),
            remember_me: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Login => self.api.auth_login(&self.login, &self.password, self.remember_me),
            Self::Message::ToggleRemember => self.remember_me = !self.remember_me,
            Self::Message::UpdateLogin(login) => self.login = login,
            Self::Message::UpdatePassword(password) => self.password = password,
        }

        false
    }

    fn view(&self) -> yew::Html {
        yew::html! {
            <div class="login">
                <form>
                    <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                    <h1 class="h3 mb-3 font-weight-normal">{ "Please sign in" }</h1>
                    <super::Alerts />
                    <label for="email" class="sr-only">{ "Username" }</label>
                    <input
                        type="email"
                        name="email"
                        class="form-control"
                        placeholder="Email address"
                        required=true
                        autofocus=true
                        oninput=self.link.callback(|e: yew::InputData| Self::Message::UpdateLogin(e.value))
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
                                value=self.remember_me
                                onclick=self.link.callback(|_| Self::Message::ToggleRemember)
                            />{ " Remember me" }
                        </label>
                    </div>
                    <a
                        class=("btn", "btn-lg", "btn-primary", "btn-block")
                        type="submit"
                        onclick=self.link.callback(|_| Self::Message::Login)
                    >{ "Sign in" }</a>
                </form>
            </div>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
