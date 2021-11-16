pub(crate) enum Message {
    Cancel,
    Create(super::form::register::Info),
    Error(oxfeed_common::Error),
    Login(super::form::login::Info),
    Logged,
    Register,
    UserCreated,
}

enum Scene {
    Login,
    Register,
}

pub(crate) struct Component {
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    link: yew::ComponentLink<Self>,
    scene: Scene,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            link,
            scene: Scene::Login,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::Cancel => {
                self.scene = Scene::Login;
                return true;
            }
            Message::Error(err) => self.event_bus.send(err.into()),
            Message::UserCreated => {
                let alert = crate::event::Alert::info("User created");
                self.event_bus.send(crate::event::Event::Alert(alert));
                self.link.send_message(Message::Cancel);
            }
            Message::Create(info) => {
                let user = oxfeed_common::new_user::Entity {
                    password: info.password,
                    email: info.email,
                };

                crate::api!(
                    self.link,
                    user_create(user) -> |_| Message::UserCreated, Message::Error
                );
            }
            Message::Login(info) => {
                let email = &info.email;
                let password = &info.password;
                let remember_me = &info.remember_me;

                crate::api!(
                    self.link,
                    auth_login(email, password, remember_me) -> |_| Message::Logged, Message::Error
                );
            }
            Message::Logged => self.event_bus.send(crate::event::Event::Logged),
            Message::Register => {
                self.scene = Scene::Register;
                return true;
            }
        }

        false
    }

    fn view(&self) -> yew::Html {
        match self.scene {
            Scene::Login => yew::html! {
                <div class="login">
                    <form>
                        <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                        <h1 class="h3 mb-3 fw-normal">{ "Please sign in" }</h1>
                        <super::Alerts />
                        <super::form::Login on_submit=self.link.callback(Message::Login) />
                        { "Don't have an account yet?" }
                        <a href="#" onclick=self.link.callback(|_| Message::Register)>{ "Register now" }</a>
                    </form>
                </div>
            },
            Scene::Register => yew::html! {
                <div class="login">
                    <form>
                        <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                        <h1 class="h3 mb-3 fw-normal">{ "Register" }</h1>
                        <super::Alerts />
                        <super::form::Register on_submit=self.link.callback(Message::Create) />
                        { "Already have login and password?" }
                        <a href="#" onclick=self.link.callback(|_| Message::Cancel)>{ "Log in" }</a>
                    </form>
                </div>
            },
        }
    }

    crate::change!();
}
