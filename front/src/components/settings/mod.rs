mod opml;
mod webhooks;

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
                <div class="card">
                    <div class="card-header">
                        { "Webhooks" }
                        <span class="help">
                            <crate::components::Svg icon="question-circle" size=16 />
                            <crate::components::Popover
                                title="What is a webhook?".to_string()
                                text="
                                A webhook is an URL called when a new item is fetched.<br />
                                This URL is called via POST method and the new item will be pass as json body.
                                "
                                position="end"
                            />
                        </span>
                    </div>
                    <div class="card-body">
                        <webhooks::Component />
                    </div>
                </div>
                <div class="card">
                    <div class="card-header">{ "OPML" }</div>
                    <div class="card-body">
                        <opml::Component />
                    </div>
                </div>
            </>
        }
    }

    crate::change!();
}
