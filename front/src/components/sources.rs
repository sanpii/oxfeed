#[derive(Default)]
enum Scene {
    Add,
    #[default]
    View,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::HtmlResult {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);
    let pagination = yew::use_state(|| elephantry_extras::Pagination::from(crate::Location::new()));
    let filter = yew::use_memo(props.clone(), |props| props.filter.clone());
    let need_update = yew::use_memo(context.clone(), |context| context.need_update);
    let pager = yew::suspense::use_future_with(
        (filter.clone(), pagination.clone(), need_update),
        |deps| async move {
            if deps.0.is_empty() {
                crate::Api::sources_all(&deps.1).await.unwrap()
            } else {
                crate::Api::sources_search(&deps.0, &deps.1).await.unwrap()
            }
        },
    )?;

    let on_add = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::Add);
        })
    };

    let on_cancel = {
        let scene = scene.clone();

        yew::Callback::from(move |_| {
            scene.set(Scene::View);
        })
    };

    let on_submit = {
        let context = context.clone();
        let scene = scene.clone();

        yew::Callback::from(move |source| {
            let context = context.clone();

            yew::suspense::Suspension::from_future(async move {
                crate::Api::sources_create(&source).await.unwrap();
                context.dispatch(crate::Action::NeedUpdate);
            });
            scene.set(Scene::View);
        })
    };

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

    let add = match *scene {
        Scene::View => yew::html! {
            <a
                class={ yew::classes!("btn", "btn-primary") }
                title="Add"
                onclick={ on_add }
            >
                <super::Svg icon="plus" size=24 />
                { "Add" }
            </a>
        },
        Scene::Add => yew::html! {
            <ul class="list-group">
                <li class="list-group-item">
                    <super::form::Source
                        source={ oxfeed_common::source::Entity::default() }
                        {on_cancel}
                        {on_submit}
                    />
                </li>
            </ul>
        },
    };

    let html = yew::html! {
        <>
            { add }
            <ul class="list-group">
            {
                for pager.iterator.iter().map(|item| {
                    yew::html! {
                        <li class="list-group-item">
                            <crate::components::Source value={ item.clone() } />
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
