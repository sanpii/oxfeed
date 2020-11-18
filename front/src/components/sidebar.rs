pub(crate) struct Component;

impl yew::Component for Component {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let links = [
            ("book", "/unread", "Unread"),
            ("diagram-3", "/sources", "Sources"),
        ];
        let router = yew_router::service::RouteService::<()>::new();
        let current_url = router.get_path();

        yew::html! {
            <ul class="nav flex-column">
            {
                for links.iter().map(|link| yew::html! {
                    <li class="nav-item">
                        <a
                            href={ link.1 }
                            class=if link.1 == current_url { "nav-link active" } else { "nav-link" }
                        >
                            <super::Svg icon=link.0 size=16 />
                            { link.2 }
                        </a>
                    </li>
                })
            }
            </ul>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
