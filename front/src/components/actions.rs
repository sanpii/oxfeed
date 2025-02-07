#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    #[prop_or_default]
    pub inline: bool,
    pub favorite: bool,
    pub read: bool,
    pub on_favorite: yew::Callback<()>,
    pub on_read: yew::Callback<()>,
}

struct Icon {
    label: &'static str,
    icon: &'static str,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let read_icon = yew::use_memo(props.read, |read| {
        if *read {
            Icon {
                label: "Mark as unread",
                icon: "eye-slash",
            }
        } else {
            Icon {
                label: "Mark as read",
                icon: "eye",
            }
        }
    });

    let favorite_icon = yew::use_memo(props.favorite, |favorite| {
        if *favorite {
            Icon {
                label: "Removes from favorites",
                icon: "star-fill",
            }
        } else {
            Icon {
                label: "Adds to favorites",
                icon: "star",
            }
        }
    });

    let on_favorite =
        yew_callback::callback!(on_favorite = props.on_favorite, move |_| on_favorite
            .emit(()));
    let on_read = yew_callback::callback!(on_read = props.on_read, move |_| on_read.emit(()));

    if props.inline {
        yew::html! {
            <div class="actions inline">
                <span class="read" onclick={ on_read } title={ read_icon.label }>
                    <super::Svg icon={ read_icon.icon } size=24 />
                </span>
                <span class="favorite" onclick={ on_favorite } title={ favorite_icon.label }>
                    <super::Svg icon={ favorite_icon.icon } size=24 />
                </span>
            </div>
        }
    } else {
        yew::html! {
            <div class="actions">
                <button class="btn btn-outline-primary" onclick={ on_read }>
                    <super::Svg icon={ read_icon.icon } size=24 />
                    { read_icon.label }
                </button>
                <button class="btn btn-outline-warning" onclick={ on_favorite }>
                    <super::Svg icon={ favorite_icon.icon } size=24 />
                    { favorite_icon.label }
                </button>
            </div>
        }
    }
}
