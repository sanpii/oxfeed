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
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);
    let pagination = yew::use_state(|| elephantry_extras::Pagination::from(crate::Location::new()));
    let filter = yew::use_memo(props.clone(), |props| props.filter.clone());
    let need_update = yew::use_memo(context.clone(), |context| context.need_update);
    let pager = yew::use_state(|| None);

    {
        let context = context.clone();
        let pager = pager.clone();

        yew::use_effect_with(
            (filter.clone(), pagination.clone(), need_update),
            move |deps| {
                let context = context.clone();
                let deps = deps.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let new_pager = if deps.0.is_empty() {
                        crate::api::call!(context, sources_all, &deps.1)
                    } else {
                        crate::api::call!(context, sources_search, &deps.0, &deps.1)
                    };
                    pager.set(Some(new_pager));
                });
            },
        );
    }

    let on_add = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::Add);
    });

    let on_cancel = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::View);
    });

    let on_submit = yew_callback::callback!(context, scene, move |source| {
        let context = context.clone();

        wasm_bindgen_futures::spawn_local(async move {
            crate::api::call!(context, sources_create, &source);
            context.dispatch(crate::Action::NeedUpdate);
        });
        scene.set(Scene::View);
    });

    let on_page_change = yew_callback::callback!(pagination, move |page| {
        pagination.set(elephantry_extras::Pagination {
            page,
            ..*pagination
        });

        gloo::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
    });

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
                        source={ oxfeed::source::Entity::default() }
                        {on_cancel}
                        {on_submit}
                    />
                </li>
            </ul>
        },
    };

    let Some(pager) = (*pager).clone() else {
        return yew::html! { add };
    };

    if pager.iterator.is_empty() {
        return yew::html! { add };
    }

    yew::html! {
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
    }
}
