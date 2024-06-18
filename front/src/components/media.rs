#[derive(PartialEq, yew::Properties)]
pub struct Properties {
    pub inline: bool,
    pub medias: Vec<oxfeed_common::media::Entity>,
}

#[yew::function_component]
pub fn Component(props: &Properties) -> yew::Html {
    if props.medias.is_empty() {
        return "".into();
    }

    if props.inline {
        inline(props)
    } else {
        expanded(props)
    }
}

fn inline(props: &Properties) -> yew::Html {
    if props.medias.len() == 1 {
        let media = &props.medias[0];

        yew::html! {
            <a href={ media.url.clone() } target="_blank"><super::Svg icon="play-btn" size=24 /></a>
        }
    } else {
        yew::html! {
            <>
                <super::Svg icon="collection-play" size=24 />
                { popover(&props.medias) }
            </>
        }
    }
}

fn expanded(props: &Properties) -> yew::Html {
    if props.medias.len() == 1 {
        let media = &props.medias[0];

        yew::html! {
            <a class={ yew::classes!("btn", "btn-outline-secondary", "medias") } href={ media.url.to_string() } target="_blank">
                <super::Svg icon="play-btn" size=24 />
                { media.file_name().unwrap() }
            </a>
        }
    } else {
        yew::html! {
            <button class={ yew::classes!("btn", "btn-outline-secondary", "medias") }>
                <super::Svg icon="collection-play" size=24 />
                { format!("{} medias", props.medias.len()) }

                { popover(&props.medias) }
            </button>
        }
    }
}

fn popover(medias: &[oxfeed_common::media::Entity]) -> yew::Html {
    let list = medias
        .iter()
        .map(|x| {
            format!(
                "<li><a href=\"{}\" target=\"_blank\">{}</a></li>",
                x.url,
                x.file_name().unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    yew::html! {
        <super::Popover text={ format!("<ul>{list}</url>") } position="end" />
    }
}
