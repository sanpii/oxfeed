pub(crate) struct Info {
    pub email: String,
    pub password: String,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub on_submit: yew::Callback<Info>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let email = yew::use_state(String::new);
    let password = yew::use_state(String::new);

    let edit_email = crate::components::edit_cb(email.clone());
    let edit_password = crate::components::edit_cb(password.clone());

    let on_submit =
        yew_callback::callback!(email, password, on_submit = props.on_submit, move |_| {
            let info = Info {
                email: (*email).clone(),
                password: (*password).clone(),
            };
            on_submit.emit(info);
        });

    yew::html! {
        <>
            <div class="form-floating">
                <input
                    type="email"
                    name="email"
                    class="form-control"
                    placeholder="Email"
                    required=true
                    oninput={ edit_email }
                />
                <label for="email" class="form-label sr-only">{ "Email" }</label>
            </div>
            <div class="form-floating">
                <label for="password" class="form-label sr-only">{ "Password" }</label>
                <input
                    type="password"
                    name="password"
                    class="form-control"
                    placeholder="Password"
                    required=true
                    oninput={ edit_password }
                />
            </div>
            <a
                class="btn btn-lg btn-primary w-100"
                type="submit"
                onclick={ on_submit }
            >{ "Register" }</a>
        </>
    }
}
