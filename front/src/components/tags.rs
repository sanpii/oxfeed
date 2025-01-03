#[yew::function_component]
pub(crate) fn Component() -> yew::HtmlResult {
    let pagination = yew::use_state(|| elephantry_extras::Pagination::from(crate::Location::new()));
    let tags = yew::suspense::use_future_with(pagination, |pagination| async move {
        crate::Api::tags_all(&pagination).await.ok()
    })?;

    let Some(tags) = (*tags).clone() else {
        return Ok(yew::html! { <super::Empty /> });
    };

    if tags.is_empty() {
        return Ok(yew::html! { <super::Empty /> });
    }

    let max = tags.iter().map(|x| x.count).max().unwrap_or(1);

    let html = yew::html! {
        <div class={ yew::classes!("d-flex", "flex-wrap", "align-items-center", "cloud") }>
        {
            for tags.iter().map(|tag| {
                let style = format!("font-size: {}rem", tag.count as f32 / max as f32 * 5. + 1.);
                let href = format!("/search/all?tag={}", tag.name);

                yew::html! {
                    <div style={ style }>
                        <a href={ href }>
                            <crate::components::Tag value={ tag.name.clone() } />
                        </a>
                    </div>
                }
            })
        }
        </div>
    };

    Ok(html)
}
