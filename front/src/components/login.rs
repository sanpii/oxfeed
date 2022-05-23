pub(crate) enum Message {
    Cancel,
    Create(super::form::register::Info),
    Error(String),
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
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    scene: Scene,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            scene: Scene::Login,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Cancel => {
                self.scene = Scene::Login;
                should_render = true;
            }
            Message::Error(_) => (),
            Message::UserCreated => {
                let alert = crate::event::Alert::info("User created");
                self.event_bus.send(crate::Event::Alert(alert));
                ctx.link().send_message(Message::Cancel);
            }
            Message::Create(info) => {
                let user = oxfeed_common::account::Entity {
                    id: None,
                    password: info.password,
                    email: info.email,
                };

                crate::api!(
                    ctx.link(),
                    account_create(user) -> |_| Message::UserCreated
                );
            }
            Message::Login(info) => {
                let email = &info.email;
                let password = &info.password;
                let remember_me = &info.remember_me;

                crate::api!(
                    ctx.link(),
                    auth_login(email, password, remember_me) -> |_| Message::Logged
                );
            }
            Message::Logged => self.event_bus.send(crate::Event::Logged),
            Message::Register => {
                self.scene = Scene::Register;
                should_render = true;
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match self.scene {
            Scene::Login => yew::html! {
                <div class="login">
                    <form>
                        <img class="mb-4" src="/logo.png" alt="" width="72px" height="72px" />
                        <h1 class="h3 mb-3 fw-normal">{ "Please sign in" }</h1>
                        <super::Alerts />
                        <super::form::Login on_submit={ ctx.link().callback(Message::Login) } />
                        { "Don't have an account yet?" }
                        <a href="#" onclick={ ctx.link().callback(|_| Message::Register) }>{ "Register now" }</a>
                    </form>
                </div>
            },
            Scene::Register => yew::html! {
                <div class="login">
                    <form>
                        <img class="mb-4" src="/logo" alt="" width="72px" height="72px" />
                        <h1 class="h3 mb-3 fw-normal">{ "Register" }</h1>
                        <super::Alerts />
                        <super::form::Register on_submit={ ctx.link().callback(Message::Create) } />
                        { "Already have login and password?" }
                        <a href="#" onclick={ ctx.link().callback(|_| Message::Cancel) }>{ "Log in" }</a>
                    </form>
                </div>
            },
        }
    }

    crate::change!();
}
