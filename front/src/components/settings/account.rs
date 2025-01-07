#[yew::function_component]
pub(crate) fn Component() -> yew::HtmlResult {
    let account = yew::use_state(|| None);

    {
        let account = account.clone();

        yew::use_state(|| {
            wasm_bindgen_futures::spawn_local(async move {
                let new_account = crate::Api::auth()
                    .await
                    .ok()
                    .map(oxfeed_common::account::Entity::from);
                account.set(new_account);
            })
        });
    }

    let Some(account) = (*account).clone() else {
        return Ok(yew::Html::default());
    };

    let on_save = {
        yew::Callback::from(move |new_account| {
            wasm_bindgen_futures::spawn_local(async move {
                crate::Api::account_update(&new_account).await.unwrap();
                logout().await.unwrap()
            });
        })
    };

    let on_delete = {
        yew::Callback::from(move |_| {
            let message = "Would you like delete your account?";

            if gloo::dialogs::confirm(message) {
                wasm_bindgen_futures::spawn_local(async move {
                    crate::Api::account_delete().await.unwrap();
                    logout().await.unwrap()
                });
            }
        })
    };

    let html = yew::html! {
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
    };

    Ok(html)
}

async fn logout() -> oxfeed_common::Result {
    crate::Api::auth_logout().await?;
    crate::Api::auth().await?;

    Ok(())
}
