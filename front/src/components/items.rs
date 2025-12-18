#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub filter: crate::Filter,
    pub kind: String,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let filter = yew::use_memo(props.clone(), |props| props.filter.clone());
    let kind = yew::use_memo(props.clone(), |props| props.kind.clone());
    let need_update = yew::use_memo(context.clone(), |context| context.need_update);
    let pagination = yew::use_state(|| elephantry_extras::Pagination::from(crate::Location::new()));
    let pager = yew::use_state(|| None);
    let bulk_active = yew::use_state(|| false);
    let bulk = yew::use_state(Vec::<oxfeed::item::Item>::new);
    let select = yew::use_state(|| false);

    let on_page_change = yew_callback::callback!(pagination, move |page| {
        pagination.set(elephantry_extras::Pagination {
            page,
            ..*pagination
        });

        gloo::utils::window().scroll_to_with_x_and_y(0.0, 0.0);
    });

    {
        let context = context.clone();
        let on_page_change = on_page_change.clone();
        let pager = pager.clone();

        yew::use_effect_with(
            (
                filter.clone(),
                kind.clone(),
                pagination.clone(),
                need_update,
            ),
            |deps| {
                let deps = deps.clone();

                yew::platform::spawn_local(async move {
                    let new_pager = if deps.0.is_empty() {
                        crate::api::call!(context, items_all, &deps.1, &deps.2)
                    } else {
                        crate::api::call!(context, items_search, &deps.1, &deps.0, &deps.2)
                    };

                    if new_pager.is_empty() && new_pager.page > 1 {
                        on_page_change.emit(new_pager.page - 1);
                    } else {
                        pager.set(Some(new_pager));
                    }
                });
            },
        );
    }

    let Some(pager) = (*pager).clone() else {
        return yew::html! { <super::Empty /> };
    };

    if pager.iterator.is_empty() {
        return yew::html! { <super::Empty /> };
    }

    let on_bulk_toggle = yew_callback::callback!(bulk_active, move |enable| {
        bulk_active.set(enable);
    });

    let on_bulk_action = yew_callback::callback!(bulk, context, move |(tag, value)| {
        for item in &(*bulk) {
            let context = context.clone();
            let item = item.clone();

            yew::platform::spawn_local(async move {
                crate::api::call!(context, items_tag, &item.id, tag, value);
                context.dispatch(crate::Action::NeedUpdate);
            });

            bulk.set(Vec::new());
        }
    });

    let on_item_toggle = yew_callback::callback!(bulk, move |(item, selected)| {
        let mut new_value = (*bulk).clone();

        if selected {
            new_value.push(item);
        } else {
            new_value.retain(|x| x == &item);
        }

        bulk.set(new_value);
    });

    yew::html! {
        <>
            <ul class="list-group">
                <super::BulkActions
                    active={ *bulk_active }
                    on_action={ on_bulk_action }
                    on_toggle={ on_bulk_toggle }
                />

                {
                    for pager.iterator.iter().map(move |item| {
                        let action_end = super::swipe::Action {
                            active: item.favorite,
                            callback: crate::toggle!(favorite, item, context),
                            color: "--bs-orange",
                            icon: ("star", "star-fill"),
                            id: item.id,
                        };

                        let action_start = super::swipe::Action {
                            active: item.read,
                            callback: crate::toggle!(read, item, context),
                            color: "--bs-blue",
                            icon: ("eye-slash", "eye"),
                            id: item.id,
                        };

                        yew::html! {
                            <li class="list-group-item">
                                <super::Swipe {action_end} {action_start}>
                                    <crate::components::Item
                                        value={ item.clone() }
                                        bulk_enable={ *bulk_active }
                                        select={ *select }
                                        on_toggle={ on_item_toggle.clone() }
                                    />
                                </super::Swipe>
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
