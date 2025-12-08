mod bar;

pub(crate) use bar::Component as Bar;

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub kind: String,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let route = yew::use_memo(context.clone(), |context| context.route.clone());
    let filter = yew::use_memo(route.clone(), |_| crate::Filter::new());

    match props.kind.as_str() {
        "sources" => yew::html! {
            <super::Sources filter={ (*filter).clone() } />
        },
        "all" => yew::html! {
            <super::Items kind="all" filter={ (*filter).clone() } />
        },
        "favorites" => yew::html! {
            <super::Items kind="favorites" filter={ (*filter).clone() } />
        },
        "unread" => yew::html! {
            <super::Items kind="unread" filter={ (*filter).clone() } />
        },
        _ => unreachable!(),
    }
}
