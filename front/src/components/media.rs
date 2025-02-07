#[derive(PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub inline: bool,
    pub medias: Vec<oxfeed::media::Entity>,
}

#[yew::function_component]
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
            <a class="btn btn-outline-secondary medias" href={ media.url.to_string() } target="_blank">
                <super::Svg icon="play-btn" size=24 />
                { media.file_name().unwrap() }
            </a>
        }
    } else {
        yew::html! {
            <button class="btn btn-outline-secondary medias">
                <super::Popover body={ popover_text(&props.medias) }>
                    <super::Svg icon="collection-play" size=24 />
                    { format!("{} medias", props.medias.len()) }
                </super::Popover>
            </button>
        }
    }
}

fn popover_text(medias: &[oxfeed::media::Entity]) -> yew::Html {
    let mut sorted_medias = medias.to_vec();
    sorted_medias.sort();

    yew::html! {
        <ul class="list-group">
        {
            sorted_medias
                .iter()
                .map(|x| {
                    let file_name = x.file_name().unwrap();

                    yew::html_nested! {
                        <li class="list-group-item">
                            <super::Svg size=24 icon={ x.content_type.clone().unwrap_or_default() } content_type=true />
                            <a href={ x.url.clone() } target="_blank" title={ file_name.clone() }>{ file_name }</a>
                        </li>
                    }
                }).collect::<yew::Html>()
        }
        </ul>
    }
}
