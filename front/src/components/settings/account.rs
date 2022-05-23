#[derive(Clone)]
pub(crate) enum Message {
    Delete,
    Error(String),
    Logout,
    NeedUpdate,
    Update(oxfeed_common::user::Entity),
    Save(oxfeed_common::account::Entity),
}

pub(crate) struct Component {
    account: Option<oxfeed_common::account::Entity>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let component = Self { account: None };

        ctx.link().send_message(Self::Message::NeedUpdate);

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Message::Delete => {
                let message = "Would you like delete your account?";

                if gloo::dialogs::confirm(message) {
                    crate::api!(
                        ctx.link(),
                        account_delete() -> |_| Message::Logout
                    );
                }
            }
            Message::Error(_) => (),
            Message::Logout => crate::api!(
                ctx.link(),
                auth_logout() -> |_| Message::NeedUpdate
            ),
            Message::NeedUpdate => crate::api!(
                ctx.link(),
                auth() -> Message::Update
            ),
            Message::Save(account) => crate::api!(
                ctx.link(),
                account_update(account) -> |_| Message::Logout
            ),
            Message::Update(account) => {
                self.account = Some(account.into());
                should_render = true;
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let account = match &self.account {
            Some(account) => account,
            None => return "".into(),
        };

        yew::html! {
            <>
                <crate::components::form::Account
                    account={ account.clone() }
                    on_save={ ctx.link().callback(Message::Save) }
                />

                <hr />

                <a
                    class="btn btn-danger"
                    title="Delete"
                    onclick={ ctx.link().callback(|_| Message::Delete) }
                >
                    <crate::components::Svg icon="trash" size=24 />
                    { "Delete my account" }
                </a>
            </>
        }
    }

    crate::change!();
}
