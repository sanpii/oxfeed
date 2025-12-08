#[derive(Clone)]
struct Links(Vec<Link>);

impl Links {
    fn new() -> Self {
        let links = vec![
            Link {
                count: 0,
                has_error: false,
                icon: "collection",
                label: "All",
                route: super::app::Route::All,
            },
            Link {
                count: 0,
                has_error: false,
                icon: "book",
                label: "Unread",
                route: super::app::Route::Unread,
            },
            Link {
                count: 0,
                has_error: false,
                icon: "star",
                label: "Favorites",
                route: super::app::Route::Favorites,
            },
            Link {
                count: 0,
                has_error: false,
                icon: "tags",
                label: "Tags",
                route: super::app::Route::Tags,
            },
            Link {
                count: 0,
                has_error: false,
                icon: "diagram-3",
                label: "Sources",
                route: super::app::Route::Sources,
            },
            Link {
                count: 0,
                has_error: false,
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
                has_error: false,
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

    fn update_count(&mut self, counts: &oxfeed::Counts) {
        self.0[0].count = counts.all;
        self.0[1].count = counts.unread;
        self.0[2].count = counts.favorites;
        self.0[3].count = counts.tags;
        self.0[4].count = counts.sources;
        self.0[4].has_error = counts.sources_has_error;
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
    has_error: bool,
}

#[derive(Clone, PartialEq, yew::Properties)]
pub(crate) struct Properties {
    pub current_route: super::app::Route,
}

#[yew::component]
pub(crate) fn Component(props: &Properties) -> yew::Html {
    let context = crate::use_context();
    let current_route = yew::use_memo(props.clone(), |props| props.current_route.clone());
    let need_update = yew::use_memo(context.clone(), |context| context.need_update);
    let counts = yew::use_state(oxfeed::Counts::default);

    {
        let context = context.clone();
        let counts = counts.clone();

        yew::use_effect_with(need_update, move |_| {
            let counts = counts.clone();

            yew::platform::spawn_local(async move {
                counts.set(crate::api::call!(context, counts));
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

    let favicon = yew::use_memo((context.clone(), counts), |deps| {
        if deps.0.sse_error {
            "error.svg".to_string()
        } else if deps.1.unread > 0 {
            format!("unread-{}.svg", deps.1.unread)
        } else {
            "default.svg".to_string()
        }
    });

    if let Ok(Some(element)) = gloo::utils::document().query_selector("link[rel=icon]") {
        let href = format!("{}/favicon/{favicon}", env!("API_URL"));
        element.set_attribute("href", &href).ok();
    }

    let read_all = yew_callback::callback!(context, move |_| {
        let context = context.clone();

        yew::platform::spawn_local(async move {
            crate::api::call!(context, items_read);
        });
    });

    yew::html! {
        <>
            <button
                class="btn btn-primary"
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
                            <super::Svg icon={ link.icon } size=16 class={ if link.has_error { "text-danger" } else { "" } } />
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
                <li class="nav-item d-md-none" data-bs-toggle="collapse" data-bs-target="#sidebarMenu">
                    <super::Logout button=false />
                </li>
            </ul>
        </>
    }
}
