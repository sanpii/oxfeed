#[derive(Clone, Copy, Default)]
enum InputType {
    #[default]
    Password,
    Text,
}

impl InputType {
    fn icon(&self) -> &'static str {
        if matches!(self, InputType::Password) {
            "eye"
        } else {
            "eye-slash"
        }
    }
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Password => "password",
            Self::Text => "text",
        };

        f.write_str(s)
    }
}

impl std::ops::Not for InputType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Password => Self::Text,
            Self::Text => Self::Password,
        }
    }
}

pub(crate) struct Info {
    pub email: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub on_submit: yew::Callback<Info>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let email = yew::use_state(String::new);
    let input_type = yew::use_state(InputType::default);
    let password = yew::use_state(String::new);
    let remember_me = yew::use_state(|| false);

    let edit_email = crate::components::edit_cb(email.clone());
    let edit_password = crate::components::edit_cb(password.clone());
    let toggle_remember_me = crate::components::toggle_cb(remember_me.clone());

    let on_submit = yew_callback::callback!(
        email,
        password,
        remember_me,
        on_submit = props.on_submit,
        move |_| {
            let info = Info {
                email: (*email).clone(),
                password: (*password).clone(),
                remember_me: *remember_me,
            };

            on_submit.emit(info);
        }
    );

    let on_click = yew_callback::callback!(input_type, move |_| {
        input_type.set(!*input_type);
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
                    autofocus=true
                    oninput={ edit_email }
                />
                <label for="email" class="form-label sr-only">{ "Email" }</label>
            </div>
            <div class="input-group">
                <div class="form-floating">
                    <input
                        type={ input_type.to_string() }
                        name="password"
                        class="form-control"
                        placeholder="Password"
                        required=true
                        oninput={ edit_password }
                    />
                    <label for="password" class="form-label sr-only">{ "Password" }</label>
                </div>
                <span class="input-group-text" onclick={ on_click }>
                    <crate::components::Svg icon={ input_type.icon() } size=16 />
                </span>
            </div>
            <div class="checkbox">
                <label class="form-label">
                    <input
                        type="checkbox"
                        checked={ *remember_me }
                        onclick={ toggle_remember_me }
                    />{ " Remember me" }
                </label>
            </div>
            <a
                class={yew::classes!("btn", "btn-lg", "btn-primary", "w-100")}
                type="submit"
                onclick={ on_submit }
            >{ "Sign in" }</a>
        </>
    }
}
