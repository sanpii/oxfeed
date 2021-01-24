pub(crate) enum Message {
    Cancel,
    Create(super::form::register::Info),
    Login(super::form::login::Info),
    Logged,
    Register,
    UserCreated,
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::Auth => Self::Logged,
            crate::event::Api::UserCreate(_) => Self::UserCreated,
            _ => unreachable!(),
        }
    }
}

enum Scene {
    Login,
    Register,
}

pub(crate) struct Component {
    api: crate::Api<Self>,
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
            api: crate::Api::new(link.clone()),
            event_bus: crate::event::Bus::dispatcher(),
            link,
            scene: Scene::Login,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Cancel => {
                self.scene = Scene::Login;
                return true;
            }
            Self::Message::UserCreated => {
                let alert = crate::event::Alert::info("User created");
                self.event_bus.send(crate::event::Event::Alert(alert));
                self.link.send_message(Self::Message::Cancel);
            }
            Self::Message::Create(info) => {
                let user = oxfeed_common::new_user::Entity {
                    password: info.password,
                    email: info.email,
                };
                self.api.user_create(&user);
            }
            Self::Message::Login(info) => {
                self.api
                    .auth_login(&info.email, &info.password, info.remember_me)
            }
            Self::Message::Logged => (),
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
                        <h1 class="h3 mb-3 font-weight-normal">{ "Please sign in" }</h1>
                        <super::Alerts />
                        <super::form::Login on_submit=self.link.callback(|info| Self::Message::Login(info)) />
                        { "Don't have an account yet?" }
                        <a href="#" onclick=self.link.callback(|_| Self::Message::Register)>{ "Register now" }</a>
                    </form>
                </div>
            },
            Scene::Register => yew::html! {
                <div class="login">
                    <form>
                        <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                        <h1 class="h3 mb-3 font-weight-normal">{ "Register" }</h1>
                        <super::Alerts />
                        <super::form::Register on_submit=self.link.callback(|info| Self::Message::Create(info)) />
                        { "Already have login and password?" }
                        <a href="#" onclick=self.link.callback(|_| Self::Message::Cancel)>{ "Log in" }</a>
                    </form>
                </div>
            },
        }
    }

    crate::change!();
}
