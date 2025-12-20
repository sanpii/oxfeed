#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub filter: oxfeed::filter::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed::filter::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let name = yew::use_state(|| props.filter.name.clone());
    let regex = yew::use_state(|| props.filter.regex.clone());
    let is_valid = yew::use_state(|| true);
    let on_cancel = props.on_cancel.clone();

    let edit_name = crate::components::edit_cb(name.clone());
    let edit_regex = yew_callback::callback!(regex, is_valid, move |e: yew::InputEvent| {
        use yew::TargetCast as _;

        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        let value = input.value();

        if let Err(err) = regex::Regex::new(&value) {
            is_valid.set(false);
            input.set_custom_validity(&err.to_string());
        } else {
            is_valid.set(true);
            input.set_custom_validity("");
        }

        input.report_validity();

        regex.set(value);
    });

    let on_submit = yew_callback::callback!(
        name,
        filter = props.filter,
        on_submit = props.on_submit,
        regex,
        move |_| {
            let mut filter = filter.clone();
            filter.name = (*name).clone();
            filter.regex = (*regex).clone();

            on_submit.emit(filter);
        }
    );

    yew::html! {
        <form class="mb-3">
            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="title">{ "Name" }</label>
                <div class="col-sm-11">
                    <input
                        class="form-control"
                        name="name"
                        required=true
                        value={ (*name).clone() }
                        oninput={ edit_name }
                    />
                </div>
            </div>

            <div class="row mb-3">
                <label class="col-sm-1 col-form-label" for="regex">{ "Regex" }</label>
                <div class="col-sm-11">
                    <input
                        class={ yew::classes!("form-control", if !*is_valid { "is-invalid" } else { "" }) }
                        name="regex"
                        required=true
                        value={ (*regex).clone() }
                        oninput={ edit_regex }
                    />
                </div>
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
                onclick={ move |_| on_cancel.emit(()) }
            >
                <crate::components::Svg icon="x" size=24 />
                { "Cancel" }
            </a>
        </form>
    }
}
