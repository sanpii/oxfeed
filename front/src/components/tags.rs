#[yew::function_component]
pub(crate) fn Component() -> yew::Html {
    let pagination = yew::use_state(|| elephantry_extras::Pagination::from(crate::Location::new()));
    let tags = yew::use_state(|| None);

    {
        let tags = tags.clone();

        yew::use_effect_with(pagination, move |pagination| {
            let pagination = pagination.clone();
            let tags = tags.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_tags = crate::Api::tags_all(&pagination).await.ok();
                tags.set(new_tags);
            });
        });
    }

    let Some(tags) = (*tags).clone() else {
        return yew::html! { <super::Empty /> };
    };

    if tags.is_empty() {
        return yew::html! { <super::Empty /> };
    }

    let max = tags.iter().map(|x| x.count).max().unwrap_or(1);

    yew::html! {
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
    }
}
