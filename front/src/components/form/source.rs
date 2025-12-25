#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub source: oxfeed::source::Entity,
    pub filters: Vec<oxfeed::filter::Entity>,
    pub webhooks: Vec<oxfeed::webhook::Entity>,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed::source::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let source = yew::use_mut_ref(move || props.source.clone());

    let edit_title = yew_callback::callback!(source, move |e: yew::InputEvent| {
        use yew::TargetCast as _;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        input.report_validity();

        source.borrow_mut().title = input.value();
    });

    let edit_url = yew_callback::callback!(source, move |e: yew::InputEvent| {
        use yew::TargetCast as _;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        input.report_validity();

        source.borrow_mut().url = input.value();
    });

    let toggle_active = yew_callback::callback!(source, move |value| {
        source.borrow_mut().active = value;
    });

    let on_cancel = yew_callback::callback!(on_cancel = props.on_cancel, move |_| {
        on_cancel.emit(());
    });

    let on_submit = yew_callback::callback!(
        on_submit = props.on_submit,
        source,
        move |_| {
            use std::ops::Deref as _;
            on_submit.emit(source.borrow().deref().clone());
        }
    );

    let value = source.clone();
    yew::html! {
        <form>
            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="title">{ "Title" }</label>
                <div class="col-sm-11">
                    <input
                        class="form-control"
                        name="title"
                        value={ source.borrow().title.clone() }
                        oninput={ edit_title }
                    />
                    <small class="form-text text-body-secondary">{ "Leave empty to use the feed title." }</small>
                </div>
            </div>

            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="url">{ "Feed URL" }</label>
                <div class="col-sm-11">
                    <input
                        class="form-control"
                        name="url"
                        required=true
                        value={ source.borrow().url.clone() }
                        oninput={ edit_url }
                    />
                </div>
            </div>

            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="tags">{ "Tags" }</label>
                <div class="col-sm-11">
                    <super::Tags
                        values={ source.borrow().tags.clone() }
                        on_change={ yew_callback::callback!(source, move |value| source.borrow_mut().tags = value) }
                    />
                </div>
            </div>

            <div class="row mb-3">
                <div class="col-sm-11 offset-sm-1">
                    <crate::components::Switch
                        id="active"
                        active={ source.borrow().active }
                        on_toggle={ toggle_active }
                        label="active"
                    />
                </div>
            </div>

            <div class="row mb-3">
                if !props.webhooks.is_empty() {
                    <div class="col">
                        <label class="col-sm-1 col-form-label" for="webhooks">{ "Webhooks" }</label>
                        <div class="col-sm-11">
                        {
                            for props.webhooks.clone().iter().map(move |webhook| {
                                let id = webhook.id.unwrap_or_default();
                                let active = value.borrow().webhooks.contains(&id);

                                yew::html! {
                                    <crate::components::Switch
                                        id={ id.to_string() }
                                        label={ webhook.name.clone() }
                                        active={ active }
                                        on_toggle={
                                            yew_callback::callback!(value,
                                                move |active| if active {
                                                    if !value.borrow().webhooks.contains(&id) {
                                                        value.borrow_mut().webhooks.push(id);
                                                    }
                                                } else {
                                                    value.borrow_mut().webhooks.retain(|x| x != &id);
                                                }
                                            )
                                        }
                                    />
                                }
                            })
                        }
                        </div>
                    </div>
                }

                if !props.filters.is_empty() {
                    <div class="col">
                        <label class="col-sm-1 col-form-label" for="filters">{ "Filters" }</label>
                        <div class="col-sm-11">
                        {
                            for props.filters.clone().iter().map(move |filter| {
                                let id = filter.id.unwrap_or_default();
                                let active = source.borrow().filters.contains(&id);

                                yew::html! {
                                    <crate::components::Switch
                                        id={ id.to_string() }
                                        label={ filter.name.clone() }
                                        active={ active }
                                        on_toggle={
                                            yew_callback::callback!(source,
                                                move |active| if active {
                                                    if !source.borrow().filters.contains(&id) {
                                                        source.borrow_mut().filters.push(id);
                                                    }
                                                } else {
                                                    source.borrow_mut().filters.retain(|x| x != &id);
                                                }
                                            )
                                        }
                                    />
                                }
                            })
                        }
                        </div>
                    </div>
                }
            </div>

            <a
                class="btn btn-primary"
                title="Save"
                onclick={ on_submit }
            >
                <crate::components::Svg icon="check" size=24 />
                { "Save" }
            </a>

            <a
                class="btn btn-secondary"
                title="Cancel"
                onclick={ on_cancel }
            >
                <crate::components::Svg icon="x" size=24 />
                { "Cancel" }
            </a>
        </form>
    }
}
