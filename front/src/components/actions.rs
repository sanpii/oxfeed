#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    #[prop_or_default]
    pub inline: bool,
    pub favorite: bool,
    pub read: bool,
    pub on_favorite: yew::Callback<()>,
    pub on_read: yew::Callback<()>,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    let (read_label, eye) = if props.read {
        ("Mark as unread", "eye-slash")
    } else {
        ("Mark as read", "eye")
    };

    let (favorite_label, star) = if props.favorite {
        ("Removes from favorites", "star-fill")
    } else {
        ("Adds to favorites", "star")
    };

    let on_favorite = crate::cb!(props.on_favorite);
    let on_read = crate::cb!(props.on_read);

    if props.inline {
        yew::html! {
            <div class={ yew::classes!("actions", "inline") }>
                <span class="read" onclick={ on_read } title={ read_label }>
                    <super::Svg icon={ eye } size=24 />
                </span>
                <span class="favorite" onclick={ on_favorite } title={ favorite_label }>
                    <super::Svg icon={ star } size=24 />
                </span>
            </div>
        }
    } else {
        yew::html! {
            <div class="actions">
                <button class={ yew::classes!("btn", "btn-outline-secondary") } onclick={ on_read }>
                    <super::Svg icon={ eye } size=24 />
                    { read_label }
                </button>
                <button class={ yew::classes!("btn", "btn-outline-warning") } onclick={ on_favorite }>
                    <super::Svg icon={ star } size=24 />
                    { favorite_label }
                </button>
            </div>
        }
    }
}
