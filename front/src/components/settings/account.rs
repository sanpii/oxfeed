#[yew::function_component]
pub(crate) fn Component() -> yew::HtmlResult {
    let res = yew::suspense::use_future(|| async move { crate::Api::auth().await })?;
    let account = match *res {
        Ok(ref account) => oxfeed_common::account::Entity::from(account.clone()),
        Err(_) => return Ok(yew::Html::default()),
    };

    let on_save = {
        yew::Callback::from(move |new_account| {
            yew::suspense::Suspension::from_future(async move {
                crate::Api::account_update(&new_account).await.unwrap();
                logout().await.unwrap()
            });
        })
    };

    let on_delete = {
        yew::Callback::from(move |_| {
            let message = "Would you like delete your account?";

            if gloo::dialogs::confirm(message) {
                yew::suspense::Suspension::from_future(async move {
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
                on_save={ on_save }
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
