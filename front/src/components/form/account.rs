pub(crate) enum Message {
    Save,
    UpdateEmail(String),
    UpdatePassword(String),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub account: oxfeed_common::account::Entity,
    pub on_save: yew::Callback<oxfeed_common::account::Entity>,
}

pub(crate) struct Component {
    props: Properties,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut should_render = false;

        match msg {
            Self::Message::Save => self.props.on_save.emit(self.props.account.clone()),
            Self::Message::UpdateEmail(email) => {
                self.props.account.email = email;
                should_render = true;
            }
            Self::Message::UpdatePassword(password) => {
                self.props.account.password = password;
                should_render = true;
            }
        }

        should_render
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <form>
                <div class="from-group">
                    <label for="email">{ "Email" }</label>
                    <input
                        class="form-control"
                        name="email"
                        value={ self.props.account.email.clone() }
                        required=true
                        oninput={ ctx.link().callback(|e: yew::InputEvent| {
                            use yew::TargetCast;

                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            Message::UpdateEmail(input.value())
                        }) }
                    />
                </div>

                <div class="from-group">
                    <label for="password">{ "Password" }</label>
                    <input
                        class="form-control"
                        type="password"
                        name="password"
                        oninput={ ctx.link().callback(|e: yew::InputEvent| {
                            use yew::TargetCast;

                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            Message::UpdatePassword(input.value())
                        }) }
                    />
                </div>

                <a
                    class="btn btn-primary"
                    title="Save"
                    onclick={ ctx.link().callback(|_| Self::Message::Save) }
                >
                    <crate::components::Svg icon="check" size=24 />
                    { "Save" }
                </a>
            </form>
        }
    }

    crate::change!();
}
