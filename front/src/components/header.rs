#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub current_route: super::app::Route,
}

pub struct Component {
    current_route: super::app::Route,
}

impl yew::Component for Component {
    type Message = ();
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            current_route: ctx.props().current_route.clone(),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        let filter = crate::Filter::new();

        yew::html! {
            <>
                <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3" href="#">{{ "Oxfeed" }}</a>
                <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <super::search::Bar current_route={ self.current_route.clone() } filter={ filter } />
                <super::Logout button=true />
            </>
        }
    }

    crate::change!(current_route);
}
