#[derive(Clone, Copy, Default)]
enum Scene {
    Add,
    #[default]
    View,
}

#[yew::component]
pub(crate) fn Component() -> yew::Html {
    let context = crate::use_context();
    let scene = yew::use_state(Scene::default);
    let filters = yew::use_state(Vec::new);

    {
        let context = context.clone();
        let filters = filters.clone();

        yew::use_effect_with((), move |_| {
            let context = context.clone();
            let filters = filters.clone();

            yew::platform::spawn_local(async move {
                let new_webhooks = crate::api::call!(context, filters_all);
                filters.set(new_webhooks);
            });
        });
    }

    let on_add = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::Add);
    });

    let on_cancel = yew_callback::callback!(scene, move |_| {
        scene.set(Scene::View);
    });

    let on_delete = yew_callback::callback!(filters, move |id| {
        let new_filters = filters
            .iter()
            .filter(|x| x.id != Some(id))
            .cloned()
            .collect();
        filters.set(new_filters);
    });

    let on_save = yew_callback::callback!(filters, move |filter: oxfeed::filter::Entity| {
        let mut new_filters = (*filters).clone();

        if let Some(old) = new_filters.iter_mut().find(|x| x.id == filter.id) {
            *old = filter;
        } else {
            new_filters.insert(0, filter);
        }

        new_filters.sort_by(|a, b| a.name.cmp(&b.name));

        filters.set(new_filters);
    });

    let on_submit = yew_callback::callback!(context, on_save, scene, move |filter| {
        let context = context.clone();
        let on_save = on_save.clone();

        yew::platform::spawn_local(async move {
            crate::api::call!(context, filters_create, &filter);
            on_save.emit(filter);
        });

        scene.set(Scene::View);
    });

    yew::html! {
        <>
            if matches!(*scene, Scene::View) {
                <a
                    class="btn btn-primary"
                    title="Add"
                    onclick={ on_add }
                >
                    <crate::components::Svg icon="plus" size=24 />
                    { "Add" }
                </a>
            }
            <ul class="list-group">
                if matches!(*scene, Scene::Add) {
                    <li class="list-group-item">
                        <crate::components::form::Filter
                            filter={ oxfeed::filter::Entity::default() }
                            {on_cancel}
                            {on_submit}
                        />
                    </li>
                }

                for filter in filters.iter() {
                    <li class="list-group-item">
                        <crate::components::Filter
                            value={ filter.clone() }
                            on_delete={ on_delete.clone() }
                            on_save={ on_save.clone() }
                        />
                    </li>
                }
            </ul>
        </>
    }
}
