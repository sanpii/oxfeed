#[macro_export]
macro_rules! toggle {
    ($name:ident, $item:ident, $context:ident) => {{
        let item = $item.clone();
        let context = $context.clone();

        yew::Callback::from(move |_| {
            let item = item.clone();
            let context = context.clone();

            wasm_bindgen_futures::spawn_local(async move {
                $crate::api::call!(context, items_tag, &item.id, stringify!($name), !item.$name);
                context.dispatch($crate::Action::NeedUpdate);
            });
        })
    }};
}

#[derive(Clone, Copy, Default, PartialEq)]
enum Scene {
    #[default]
    Hidden,
    Expanded,
}

impl std::ops::Not for Scene {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Expanded => Self::Hidden,
            Self::Hidden => Self::Expanded,
        }
    }
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub value: oxfeed::item::Item,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let content = yew::use_state(|| None::<String>);
    let item = yew::use_memo(props.clone(), |props| props.value.clone());
    let scene = yew::use_state(Scene::default);

    let published_ago = chrono_humanize::HumanTime::from(item.published);
    let published_class = if (*item).in_future() {
        "text-body-tertiary"
    } else {
        "text-body-secondary"
    };

    let caret = match *scene {
        Scene::Expanded => "chevron-up",
        Scene::Hidden => "chevron-down",
    };

    let title = gloo::utils::document().create_element("span").unwrap();
    title.set_inner_html(&item.title);

    let content_div = gloo::utils::document().create_element("div").unwrap();
    content_div.set_inner_html(content.as_ref().unwrap_or(&"Loading...".to_string()));

    let icon = if let Some(icon) = &item.icon {
        format!("{}{icon}", env!("API_URL"))
    } else {
        "/1px.png".to_string()
    };

    let on_favorite = toggle!(favorite, item, context);
    let on_read = toggle!(read, item, context);

    let toggle_content = yew_callback::callback!(content, context, item, scene, move |_| {
        scene.set(!*scene);

        if content.is_none() {
            let content = content.clone();
            let context = context.clone();
            let item = item.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let item_id = &item.id;

                content.set(Some(crate::api::call!(context, items_content, item_id)));
            });
        }
    });

    yew::html! {
        <>
            <img src={ icon } width="16" height="16" />
            <a href={ item.link.clone() } target="_blank">
                { yew::virtual_dom::VNode::VRef(title.into()) }
            </a>
            {
                for item.tags.iter().map(|tag| {
                    yew::html! { <super::Tag value={ tag.clone() } /> }
                })
            }

            if *scene == Scene::Hidden {
                if !item.media.is_empty() {
                    <span class="text-body-secondary">{ "· " }</span>
                    <span class="medias" title="Medias">
                        <super::Media inline=true medias={ item.media.clone() } />
                    </span>
                } else {
                    <></>
                }
            } else {
                <></>
            }

            <span class="text-body-secondary">{ "· " }{ &item.source }</span>

            <div class="float-end">
                <span class={ yew::classes!(published_class, "d-none", "d-md-inline") }>{ &published_ago.to_string() }</span>
                <span onclick={ toggle_content }>
                    <super::Svg icon={ caret } size=24 />
                </span>
            </div>
            <div class="float-end">
                {
                    if *scene == Scene::Hidden {
                        let on_favorite = on_favorite.clone();
                        let on_read = on_read.clone();

                        yew::html! {
                            <super::Actions
                                inline=true
                                read={ item.read }
                                {on_read}
                                favorite={ item.favorite }
                                {on_favorite}
                            />
                        }
                    } else {
                        yew::Html::default()
                    }
                }
                {
                    if *scene == Scene::Hidden && item.favorite {
                        yew::html! {
                            <div class="favorite">
                                <super::Svg icon="star-fill" size=24 />
                            </div>
                        }
                    } else {
                        yew::Html::default()
                    }
                }
            </div>
            {
                if *scene == Scene::Expanded {
                    yew::html! {
                        <>
                            { yew::virtual_dom::VNode::VRef(content_div.into()) }

                            <super::Media
                                inline=false
                                medias={ item.media.clone() }
                            />

                            <hr />

                            <super::Actions
                                read={ item.read }
                                {on_read}
                                favorite={ item.favorite }
                                {on_favorite}
                            />
                        </>
                    }
                } else {
                    yew::Html::default()
                }
            }
        </>
    }
}
