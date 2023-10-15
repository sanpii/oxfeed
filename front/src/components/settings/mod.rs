mod account;
mod opml;
mod webhooks;

#[yew::function_component]
pub fn Component() -> yew::Html {
    yew::html! {
        <>
            <div class="card">
                <div class="card-header">
                    { "Webhooks" }
                    <span class="help">
                        <crate::components::Svg icon="question-circle" size=16 />
                        <crate::components::Popover
                            title={ "What is a webhook?".to_string() }
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
            <div class="card">
                <div class="card-header">{ "Account" }</div>
                <div class="card-body">
                    <account::Component />
                </div>
            </div>
        </>
    }
}
