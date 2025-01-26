#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub on_select: yew::Callback<String>,
    #[prop_or_default]
    pub on_delete: yew::Callback<()>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let active = yew::use_state(|| None::<usize>);
    let input_ref = yew::NodeRef::default();
    let on_delete = props.on_delete.clone();
    let on_select = props.on_select.clone();
    let terms = yew::use_state(Vec::new);
    let value = yew::use_state(String::new);

    let select = yew_callback::callback!(active, on_select, terms, value, move |selected| {
        on_select.emit(selected);
        active.set(None);
        terms.set(Vec::new());
        value.set(String::new());
    });

    let on_input =
        yew_callback::callback!(active, context, terms, value, move |e: yew::InputEvent| {
            use yew::TargetCast as _;

            let context = context.clone();

            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            value.set(input.value());

            if value.is_empty() {
                terms.set(Vec::new());
                active.set(None);
            } else {
                let pagination = elephantry_extras::Pagination::new();
                let filter = (*value).clone().into();
                let terms = terms.clone();

                yew::platform::spawn_local(async move {
                    let pager = crate::api::call!(context, tags_search, &filter, &pagination);
                    terms.set(pager.iterator);
                });
            }
        });

    let on_keydown = yew_callback::callback!(
        active,
        select,
        terms,
        value,
        move |e: yew::KeyboardEvent| {
            match e.key().as_str() {
                "ArrowDown" => {
                    let new_value = if let Some(active) = *active {
                        Some((active + 1) % terms.len())
                    } else {
                        Some(0)
                    };

                    active.set(new_value);
                }
                "ArrowUp" => {
                    let new_value = if let Some(active) = *active {
                        Some(active.checked_sub(1).unwrap_or(terms.len() - 1))
                    } else {
                        Some(terms.len() - 1)
                    };

                    active.set(new_value);
                }
                "Backspace" => {
                    if value.is_empty() {
                        on_delete.emit(());
                    }
                }
                "Enter" => {
                    if let Some(active) = *active {
                        select.emit(terms[active].clone());
                    } else {
                        select.emit((*value).clone());
                    }
                }
                "Escape" => {
                    terms.set(Vec::new());
                    active.set(None);
                }
                _ => (),
            };
        }
    );

    yew::html! {
        <div class="autocomplete">
            <input
                type="text"
                ref={ input_ref.clone() }
                value={ (*value).clone() }
                oninput={ on_input }
                onkeydown={ on_keydown }
            />
            {
                if terms.is_empty() {
                    yew::Html::default()
                } else {
                    yew::html! {
                        <div class="list-group">
                        {
                            for terms.clone().iter().enumerate().map(|(idx, term)| {
                                let active = active.clone();
                                let select = select.clone();
                                let terms = terms.clone();

                                yew::html! {
                                    <div
                                        class={ yew::classes!("list-group-item", "list-group-item-action", if (*active) == Some(idx) { "active" } else { "" }) }
                                        onclick={ yew_callback::callback!(move |_| { select.emit(terms[idx].clone()); }) }
                                    >{ term }</div>
                                }
                            })
                        }
                        </div>
                    }
                }
            }
        </div>
    }
}
