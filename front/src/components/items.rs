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
    let pager = yew::use_state(crate::Pager::default);
    let bulk_active = yew::use_state(|| false);

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
                        let items = new_pager
                            .iterator
                            .into_iter()
                            .map(|x| (x, false))
                            .collect::<Vec<_>>();

                        let enumerate_pager = crate::Pager {
                            iterator: items,
                            result_count: new_pager.result_count,
                            result_min: new_pager.result_min,
                            result_max: new_pager.result_max,
                            last_page: new_pager.last_page,
                            page: new_pager.page,
                            has_next_page: new_pager.has_next_page,
                            has_previous_page: new_pager.has_previous_page,
                            count: new_pager.count,
                            max_per_page: new_pager.max_per_page,
                            base_url: new_pager.base_url,
                        };
                        pager.set(enumerate_pager);
                    }
                });
            },
        );
    }

    let on_bulk_toggle = yew_callback::callback!(bulk_active, move |enable| {
        bulk_active.set(enable);
    });

    let on_bulk_action = yew_callback::callback!(context, pager, move |(tag, value)| {
        let mut new_pager = (*pager).clone();

        new_pager.iterator.iter_mut().filter(|x| x.1).for_each(|x| {
            let item = x.clone();
            let context = context.clone();

            yew::platform::spawn_local(async move {
                crate::api::call!(context, items_tag, &item.0.id, tag, value);
                context.dispatch(crate::Action::NeedUpdate);
            });

            x.1 = false;
        });

        pager.set(new_pager);
    });

    let on_bulk_select = yew_callback::callback!(bulk_active, pager, move |selection| {
        let mut new_pager = (*pager).clone();
        let active = matches!(selection, super::bulk_actions::Selection::All);

        new_pager.iterator.iter_mut().for_each(|x| x.1 = active);
        pager.set(new_pager);

        bulk_active.set(true);
    });

    let on_item_toggle = yew_callback::callback!(pager, move |event: super::item::ToggleEvent| {
        let mut new_value = (*pager).clone();

        select(&mut new_value.iterator, event);

        pager.set(new_value);
    });

    let timeout = yew::use_mut_ref(|| None::<gloo::timers::callback::Timeout>);
    let on_touch_start = yew_callback::callback!(bulk_active, timeout, move |_| {
        let bulk_active = bulk_active.clone();

        let t = gloo::timers::callback::Timeout::new(1_500, move || {
            bulk_active.set(true);
        });

        *timeout.borrow_mut() = Some(t);
    });
    let on_touch_end = yew_callback::callback!(timeout, move |_| {
        (*timeout).take().unwrap().cancel();
    });

    let webhook_response = yew::use_state(|| None);
    let on_bulk_webhook =
        yew_callback::callback!(context, pager, webhook_response, move |id: uuid::Uuid| {
            let mut new_pager = (*pager).clone();

            new_pager.iterator.iter_mut().filter(|x| x.1).for_each(|x| {
                let context = context.clone();
                let item = x.0.clone();
                let webhook_response = webhook_response.clone();

                yew::platform::spawn_local(async move {
                    let response = crate::api::call!(context, webhooks_execute, &id, &item);
                    webhook_response.set(Some(response));
                });

                x.1 = false;
            });

            pager.set(new_pager);
        });

    let on_close = yew_callback::callback!(webhook_response, move |_| {
        webhook_response.set(None);
    });

    if pager.iterator.is_empty() {
        return yew::html! {
            <>
                <super::BulkActions
                    disabled=true
                    active={ *bulk_active }
                    on_action={ on_bulk_action }
                    on_select={ on_bulk_select }
                    on_toggle={ on_bulk_toggle }
                    on_webhook={ on_bulk_webhook }
                />
                <super::Empty />
            </>
        };
    }

    yew::html! {
        <>
            <super::BulkActions
                disabled=false
                active={ *bulk_active }
                on_action={ on_bulk_action }
                on_select={ on_bulk_select }
                on_toggle={ on_bulk_toggle }
                on_webhook={ on_bulk_webhook }
            />

            if let Some(ref response) = *webhook_response {
                <div class="modal d-block" tabindex="-1">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                { "Status code: " }{ &response.status }
                            </div>
                            <div class="modal-body">
                                <pre><code>{ &response.body }</code></pre>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" onclick={ on_close }>{ "Close" }</button>
                            </div>
                        </div>
                    </div>
                </div>
            }

            <ul class="list-group" ontouchstart={ on_touch_start } ontouchend={ on_touch_end }>
                {
                    for pager.iterator.iter().map(move |item| {
                        let action_end = super::swipe::Action {
                            active: item.0.favorite,
                            callback: crate::toggle!(favorite, item.0, context),
                            color: "--bs-orange",
                            icon: ("star", "star-fill"),
                            id: item.0.id,
                        };

                        let action_start = super::swipe::Action {
                            active: item.0.read,
                            callback: crate::toggle!(read, item.0, context),
                            color: "--bs-blue",
                            icon: ("eye-slash", "eye"),
                            id: item.0.id,
                        };

                        yew::html! {
                            <li class="list-group-item">
                                <super::Swipe {action_end} {action_start}>
                                    <crate::components::Item
                                        value={ item.0.clone() }
                                        bulk_enable={ *bulk_active }
                                        select={ item.1 }
                                        on_toggle={ on_item_toggle.clone() }
                                    />
                                </super::Swipe>
                            </li>
                        }
                    })
                }
            </ul>
            <elephantry_extras::yew::Pager
                value={ elephantry_extras::Pager::from((*pager).clone()) }
                onclick={ on_page_change }
            />
        </>
    }
}

fn select(items: &mut [(oxfeed::item::Item, bool)], event: super::item::ToggleEvent) {
    let Some(current) = items.iter().position(|x| x.0 == event.item) else {
        return;
    };

    let range = if event.multiple {
        let near = near(current, items);

        if current < near {
            current..=near
        } else {
            near..=current
        }
    } else {
        current..=current
    };

    for x in range {
        items[x].1 = event.active;
    }
}

fn near(current: usize, items: &[(oxfeed::item::Item, bool)]) -> usize {
    if let Some((pos, _)) = items.iter().enumerate().skip(current).find(|(_, x)| x.1) {
        pos
    } else if let Some((pos, _)) = items
        .iter()
        .enumerate()
        .rev()
        .skip(items.len() - current)
        .find(|(_, x)| x.1)
    {
        pos
    } else {
        0
    }
}
