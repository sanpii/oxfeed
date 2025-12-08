#[yew::component]
pub fn Component() -> yew::Html {
    let context = crate::use_context();
    let theme = yew::use_memo(context.clone(), |context| context.theme);

    let on_click = yew_callback::callback!(context, move |value: crate::context::Theme| {
        use gloo::storage::Storage as _;

        match gloo::storage::LocalStorage::set("theme", value) {
            Ok(_) => context.dispatch(crate::Action::Theme(value)),
            Err(err) => context.dispatch(oxfeed::Error::from(err).into()),
        }
    });

    yew::html! {
        <ul class="navbar-nav">
            <li class="nav-item dropdown">
                <button class="btn btn-dark dropdown-toggle" data-bs-toggle="dropdown">
                    <super::Svg icon={ theme.icon() } size=16 />
                </button>
                <ul class="dropdown-menu dropdown-menu-dark">
                {
                    crate::context::Theme::all().into_iter().map(|x| {
                        let on_click = on_click.clone();

                        yew::html! {
                            <li>
                                <button
                                    class={ yew::classes!("dropdown-item", if *theme == x { "active" } else { "" }) }
                                    onclick={ move |_| on_click.emit(x) }
                                >
                                    <super::Svg icon={ x.icon() } size=16 />{ x }
                                </button>
                            </li>
                        }
                    }).collect::<Vec<yew::Html>>()
                }
                </ul>
            </li>
        </ul>
    }
}
