#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub webhook: oxfeed::webhook::Entity,
    pub on_cancel: yew::Callback<()>,
    pub on_submit: yew::Callback<oxfeed::webhook::Entity>,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let name = yew::use_state(|| props.webhook.name.clone());
    let mark_read = yew::use_state(|| props.webhook.mark_read);
    let url = yew::use_state(|| props.webhook.url.clone());
    let on_cancel = props.on_cancel.clone();

    let edit_name = crate::components::edit_cb(name.clone());
    let edit_url = crate::components::edit_cb(url.clone());

    let toggle_mark_read = {
        let mark_read = mark_read.clone();

        yew::Callback::from(move |value| {
            mark_read.set(value);
        })
    };

    let on_submit = {
        let mark_read = mark_read.clone();
        let name = name.clone();
        let webhook = props.webhook.clone();
        let on_submit = props.on_submit.clone();
        let url = url.clone();

        yew::Callback::from(move |_| {
            let mut webhook = webhook.clone();
            webhook.name = (*name).clone();
            webhook.url = (*url).clone();
            webhook.mark_read = *mark_read;

            on_submit.emit(webhook);
        })
    };

    yew::html! {
        <form>
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
                <label class="col-sm-1 col-form-label" for="url">{ "URL" }</label>
                <div class="col-sm-11">
                    <input
                        class="form-control"
                        name="url"
                        required=true
                        value={ (*url).clone() }
                        oninput={ edit_url }
                    />
                </div>
            </div>

            <div class="row mb-3">
                <div class="col-sm-11 offset-sm-1">
                    <crate::components::Switch
                        id="mark_read"
                        label="Mark item as read after webhook call"
                        active={ *mark_read }
                        on_toggle={ toggle_mark_read }
                    />
                </div>
            </div>

            <a
                class={ yew::classes!("btn", "btn-primary") }
                title="Save"
                onclick={ on_submit }
            >
                <crate::components::Svg icon="check" size=24 />
                { "Save" }
            </a>

            <a
                class={ yew::classes!("btn", "btn-secondary") }
                title="Cancel"
                onclick={ move |_| on_cancel.emit(()) }
            >
                <crate::components::Svg icon="x" size=24 />
                { "Cancel" }
            </a>
        </form>
    }
}
