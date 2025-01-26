#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let account = yew::use_state(|| None::<oxfeed::account::Entity>);

    {
        let account = account.clone();
        let context = context.clone();

        yew::use_state(|| {
            let context = context.clone();

            yew::platform::spawn_local(async move {
                let new_account = crate::api::call!(context, auth).into();
                account.set(Some(new_account));
            })
        });
    }

    let Some(account) = (*account).clone() else {
        return yew::Html::default();
    };

    let on_save = yew_callback::callback!(context, move |new_account| {
        let context = context.clone();

        yew::platform::spawn_local(async move {
            crate::api::call!(context, account_update, &new_account);
            logout().await.unwrap()
        });
    });

    let on_delete = yew_callback::callback!(context, move |_| {
        let context = context.clone();
        let message = "Would you like delete your account?";

        if gloo::dialogs::confirm(message) {
            yew::platform::spawn_local(async move {
                crate::api::call!(context, account_delete);
                logout().await.unwrap()
            });
        }
    });

    yew::html! {
        <>
            <crate::components::form::Account
                account={ account.clone() }
                {on_save}
            />

            <hr />

            <a
                class="btn btn-danger"
                title="Delete"
                onclick={ on_delete }
            >
                <crate::components::Svg icon="trash" size=24 />
                { "Delete my account" }
            </a>
        </>
    }
}

async fn logout() -> oxfeed::Result {
    crate::Api::auth_logout().await?;
    crate::Api::auth().await?;

    Ok(())
}
