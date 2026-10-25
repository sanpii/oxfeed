#[derive(PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub inline: bool,
    pub medias: Vec<oxfeed::media::Entity>,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    if props.medias.is_empty() {
        yew::Html::default()
    } else if props.inline {
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
            <super::Popover body={ popover_text(&props.medias) }>
                <super::Svg icon="collection-play" size=24 />
            </super::Popover>
        }
    }
}

fn expanded(props: &Properties) -> yew::Html {
    if props.medias.len() == 1 {
        let media = &props.medias[0];

        yew::html! {
            <a class="btn-outline theme-accent" href={ media.url.to_string() } target="_blank">
                <super::Svg icon="play-btn" size=24 />
                { media.file_name().unwrap() }
            </a>
        }
    } else {
        yew::html! {
            <super::Popover body={ popover_text(&props.medias) } class="btn-outline theme-accent">
                <super::Svg icon="collection-play" size=24 />
                { format!("{} medias", props.medias.len()) }
            </super::Popover>
        }
    }
}

fn popover_text(medias: &[oxfeed::media::Entity]) -> String {
    let mut sorted_medias = medias.to_vec();
    sorted_medias.sort();

    let mut text = "<ul class=\"list-group\">".to_string();

    for media in sorted_medias {
        let file_name = media.file_name().unwrap();

        text.push_str(&format!(
            r#"<li class="list-group-item">
                    <a href="{}" target="_blank" title="{file_name}">{file_name}</a>
                </li>"#,
            media.url
        ));
    }

    text.push_str("</ul>");

    text
}
