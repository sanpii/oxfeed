#[derive(Clone)]
struct Links(Vec<Link>);

impl Links {
    fn new() -> Self {
        let links = vec![
            Link {
                count: 0,
                icon: "collection",
                label: "All",
                route: super::app::Route::All,
            },
            Link {
                count: 0,
                icon: "book",
                label: "Unread",
                route: super::app::Route::Unread,
            },
            Link {
                count: 0,
                icon: "star",
                label: "Favorites",
                route: super::app::Route::Favorites,
            },
            Link {
                count: 0,
                icon: "tags",
                label: "Tags",
                route: super::app::Route::Tags,
            },
            Link {
                count: 0,
                icon: "diagram-3",
                label: "Sources",
                route: super::app::Route::Sources,
            },
            Link {
                count: 0,
                icon: "sliders",
                label: "Settings",
                route: super::app::Route::Settings,
            },
        ];

        Self(links)
    }

    fn add_search(&mut self, search_route: &super::app::Route) {
        if !self.has_search() {
            self.0.push(Link {
                count: 0,
                icon: "search",
                label: "Search",
                route: search_route.clone(),
            });
        }
    }

    fn remove_search(&mut self) {
        if self.has_search() {
            self.0.pop();
        }
    }

    fn has_search(&self) -> bool {
        self.0.len() != Self::new().0.len()
    }

    fn update_count(&mut self, counts: &oxfeed_common::Counts) {
        self.0[0].count = counts.all;
        self.0[1].count = counts.unread;
        self.0[2].count = counts.favorites;
        self.0[3].count = counts.tags;
        self.0[4].count = counts.sources;
    }
}

impl std::ops::Deref for Links {
    type Target = Vec<Link>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Links {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
struct Link {
    count: i64,
    icon: &'static str,
    label: &'static str,
    route: super::app::Route,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: super::app::Route,
}

#[yew::function_component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let current_route = yew::use_memo(props.clone(), |props| props.current_route.clone());
    let need_update = yew::use_memo(context.clone(), |context| context.need_update);
    let counts = yew::use_state(oxfeed_common::Counts::default);

    {
        let counts = counts.clone();

        yew::use_effect_with(need_update, move |_| {
            let counts = counts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match crate::Api::counts().await {
                    Ok(new_counts) => counts.set(new_counts),
                    Err(err) => {
                        log::error!("{err:?}");
                        panic!();
                    }
                }
            });
        });
    }

    let links = yew::use_memo((counts.clone(), current_route.clone()), |deps| {
        let mut links = Links::new();
        links.update_count(&deps.0);

        if matches!(*deps.1, super::app::Route::Search { .. }) {
            links.add_search(&current_route);
        } else {
            links.remove_search();
        }

        links
    });
    let unread = yew::use_memo(counts.clone(), |counts| counts.unread > 0);

    let favicon = yew::use_memo((context.clone(), unread), |deps| {
        if deps.0.websocket_error {
            "/favicon-error.ico"
        } else if *deps.1 {
            "/favicon-unread.ico"
        } else {
            "/favicon.ico"
        }
    });

    if let Ok(Some(element)) = gloo::utils::document().query_selector("link[rel=icon]") {
        element.set_attribute("href", *favicon).ok();
    }

    let read_all = {
        let context = context.clone();

        yew::Callback::from(move |_| {
            let context = context.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Err(err) = crate::Api::items_read().await {
                    context.dispatch(err.into());
                }
            });
        })
    };

    yew::html! {
        <>
            <button
                class={ yew::classes!("btn", "btn-primary") }
                onclick={ read_all }
            >{ "Mark all as read" }</button>
            <ul class="nav flex-column">
            {
                for links.clone().iter().map(move |link| yew::html! {
                    <li class="nav-item" data-bs-toggle="collapse" data-bs-target="#sidebarMenu">
                        <yew_router::components::Link<super::app::Route>
                            to={ link.route.clone() }
                            classes={ if link.route == *current_route { "nav-link active" } else { "nav-link" } }
                        >
                            <super::Svg icon={ link.icon } size=16 />
                            { link.label }
                            {
                                if link.count > 0 {
                                    yew::html! {
                                        <span
                                            class={ if link.route == *current_route { "badge bg-primary" } else { "badge bg-secondary" } }
                                        >{ link.count }</span>
                                    }
                                } else {
                                    yew::Html::default()
                                }
                            }
                        </yew_router::components::Link<super::app::Route>>
                    </li>
                })
            }
                <li class={ yew::classes!("nav-item", "d-md-none") } data-bs-toggle="collapse" data-bs-target="#sidebarMenu">
                    <super::Logout button=false />
                </li>
            </ul>
        </>
    }
}
