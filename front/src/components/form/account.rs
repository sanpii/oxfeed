#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub account: oxfeed::account::Entity,
    pub on_save: yew::Callback<oxfeed::account::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let account = yew::use_mut_ref(|| props.account.clone());

    let on_save = yew_callback::callback!(account, on_save = props.on_save, move |_| {
        on_save.emit(account.borrow().clone());
    });

    let edit_email = yew_callback::callback!(account, move |e: yew::InputEvent| {
        use yew::TargetCast;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        account.borrow_mut().email = input.value();
    });

    let edit_password = yew_callback::callback!(account, move |e: yew::InputEvent| {
        use yew::TargetCast;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        account.borrow_mut().password = input.value();
    });

    yew::html! {
        <form>
            <div class="from-group">
                <label for="email">{ "Email" }</label>
                <input
                    class="form-control"
                    name="email"
                    value={ account.borrow().email.clone() }
                    required=true
                    oninput={ edit_email }
                />
            </div>

            <div class="from-group">
                <label for="password">{ "Password" }</label>
                <input
                    class="form-control"
                    type="password"
                    name="password"
                    oninput={ edit_password }
                />
            </div>

            <a
                class="btn btn-primary"
                title="Save"
                onclick={ on_save }
            >
                <crate::components::Svg icon="check" size=24 />
                { "Save" }
            </a>
        </form>
    }
}
