#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
    pub kind: String,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::HtmlResult {
    let context = crate::use_context();
    let filter = yew::use_memo(props.clone(), |props| props.filter.clone());
    let kind = yew::use_memo(props.clone(), |props| props.kind.clone());
    let need_update = yew::use_memo(context.clone(), |context| context.need_update);
    let pagination = yew::use_state(|| elephantry_extras::Pagination::from(crate::Location::new()));
    let pager = yew::suspense::use_future_with(
        (
            filter.clone(),
            kind.clone(),
            pagination.clone(),
            need_update,
        ),
        |deps| async move {
            if deps.0.is_empty() {
                crate::Api::items_all(&deps.1, &deps.2).await.ok()
            } else {
                crate::Api::items_search(&deps.1, &deps.0, &deps.2)
                    .await
                    .ok()
            }
        },
    )?;

    let on_page_change = {
        let pagination = pagination.clone();

        yew::Callback::from(move |page| {
            pagination.set(elephantry_extras::Pagination {
                page,
                ..*pagination
            });

            gloo::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
        })
    };

    let Some(pager) = (*pager).clone() else {
        return Ok(yew::html! { <super::Empty /> });
    };

    if pager.iterator.is_empty() {
        return Ok(yew::html! { <super::Empty /> });
    }

    let html = yew::html! {
        <>
            <ul class="list-group">
            {
                for pager.iterator.iter().map(|item| {
                    yew::html! {
                        <li class="list-group-item">
                            <crate::components::Item value={ item.clone() } />
                        </li>
                    }
                })
            }
            </ul>
            <elephantry_extras::yew::Pager
                value={ elephantry_extras::Pager::from(pager.clone()) }
                onclick={ on_page_change }
            />
        </>
    };

    Ok(html)
}
