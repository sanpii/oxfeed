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
        use yewtil::future::LinkFuture;

        match msg {
            Self::Message::Cancel => {
                self.scene = Scene::Login;
                return true;
            }
            Self::Message::Error(err) => self.event_bus.send(err.into()),
            Self::Message::UserCreated => {
                let alert = crate::event::Alert::info("User created");
                self.event_bus.send(crate::event::Event::Alert(alert));
                self.link.send_message(Self::Message::Cancel);
            }
            Self::Message::Create(info) => {
                self.link.send_future(async {
                    let user = oxfeed_common::new_user::Entity {
                        password: info.password,
                        email: info.email,
                    };
                    crate::Api::user_create(&user)
                        .await
                        .map_or_else(Self::Message::Error, |_| Self::Message::UserCreated)
                });
            }
            Self::Message::Login(info) => {
                self.link.send_future(async move {
                    crate::Api::auth_login(&info.email, &info.password, info.remember_me)
                        .await
                        .map_or_else(Self::Message::Error, |_| Self::Message::Logged)
                });
            }
            Self::Message::Logged => self.event_bus.send(crate::event::Event::Logged),
            Self::Message::Register => {
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
                        <super::form::Login on_submit=self.link.callback(Self::Message::Login) />
                        { "Don't have an account yet?" }
                        <a href="#" onclick=self.link.callback(|_| Self::Message::Register)>{ "Register now" }</a>
                    </form>
                </div>
            },
            Scene::Register => yew::html! {
                <div class="login">
                    <form>
                        <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                        <h1 class="h3 mb-3 fw-normal">{ "Register" }</h1>
                        <super::Alerts />
                        <super::form::Register on_submit=self.link.callback(Self::Message::Create) />
                        { "Already have login and password?" }
                        <a href="#" onclick=self.link.callback(|_| Self::Message::Cancel)>{ "Log in" }</a>
                    </form>
                </div>
            },
        }
    }

    crate::change!();
}
