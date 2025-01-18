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
            for tags.iter().map(|tag| yew::html! {
                <Tag tag={ tag.clone() } {max} />
            })
        }
        </div>
    }
}

#[derive(Clone, Copy, Default)]
enum Scene {
    Edit,
    #[default]
    View,
}

#[derive(PartialEq, yew::Properties)]
struct TagProperties {
    tag: oxfeed::Tag,
    max: i64,
}

#[yew::function_component]
fn Tag(props: &TagProperties) -> yew::Html {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);
    let name = yew::use_state(|| props.tag.name.clone());

    let style = format!(
        "font-size: {}rem",
        props.tag.count as f32 / props.max as f32 * 5. + 1.
    );

    let bg_color = crate::cha::Color::from(&name);
    let color = if bg_color.is_dark() { "white" } else { "black" };
    let span_style = format!(
        "background-color: {}; color: {color}",
        bg_color.to_color_string(),
    );

    let href = format!("/search/all?tag={}", *name);

    let on_cancel = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::View);
        })
    };

    let on_edit = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::Edit);
        })
    };

    let on_save = {
        let context = context.clone();
        let scene = scene.clone();
        let name = name.clone();
        let tag = props.tag.name.clone();

        yew::Callback::from(move |_| {
            if !name.is_empty() {
                let context = context.clone();
                let name = name.clone();
                let tag = tag.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(err) = crate::Api::tags_rename(&tag, &name).await {
                        context.dispatch(err.into());
                    }
                });
                scene.set(Scene::View);
            }
        })
    };

    let edit_tag = crate::components::edit_cb(name.clone());

    match *scene {
        Scene::View => yew::html! {
            <div style={ style }>
                <a href={ href }>
                    <crate::components::Tag
                        value={ (*name).clone() }
                        editable=true
                        {on_edit}
                    />
                </a>
            </div>
        },
        Scene::Edit => yew::html! {
            <div {style}>
                <span style={ span_style } class="badge">
                    <div class="input-group">
                        <input
                            type="text"
                            value={ (*name).clone() }
                            required=true
                            oninput={ edit_tag }
                        />
                        <button class="btn btn-primary" type="button">
                            <super::Svg icon="check" size=24 on_click = { on_save } />
                        </button>
                        <button class="btn btn-danger" type="button">
                            <super::Svg icon="x" size=24 on_click = { on_cancel } />
                        </button>
                    </div>
                </span>
            </div>
        },
    }
}
