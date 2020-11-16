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
        yew::html! {
            <>
                <a class="navbar-brand col-md-3 col-lg-2 mr-0 px-3" href="#">{{ "Oxfeed" }}</a>
                <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-toggle="collapse" data-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <input class="form-control form-control-dark w-100" type="text" placeholder="Search" aria-label="Search" />
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
