pub(crate) enum Message {
    Error(oxfeed_common::Error),
    Import,
    Imported,
    Loaded(String),
    Files(Vec<gloo::file::File>),
}

pub(crate) struct Component {
    event_bus: yew_agent::Dispatcher<crate::event::Bus>,
    files: Vec<gloo::file::File>,
}

impl Component {
    fn load(&mut self, link: &yew::html::Scope<Self>) {
        for file in &self.files {
            let link = link.clone();

            gloo::file::callbacks::read_as_text(file, move |content| {
                link.send_message(Message::Loaded(content.unwrap_or_default()));
            });
        }
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        use yew_agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            files: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Error(err) => self.event_bus.send(err.into()),
            Message::Files(files) => self.files = files,
            Message::Import => self.load(ctx.link()),
            Message::Imported => {
                let alert = crate::event::Alert::info("Import done");
                self.event_bus.send(crate::Event::Alert(alert));
                self.event_bus.send(crate::Event::SettingUpdate);
            }
            Message::Loaded(content) => {
                ctx.link().send_future(async move {
                    crate::Api::opml_import(content)
                        .await
                        .map_or_else(Message::Error, |_| Message::Imported)
                });
            }
        }

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let export_url = format!("{}/opml", env!("API_URL"));

        yew::html! {
            <>
                <div class="input-group mb-3">
                    <input type="file" class="form-control" onchange={ ctx.link().callback(|e: yew::Event| {
                        use yew::TargetCast;

                        let mut files = Vec::new();
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();

                        if let Some(file_list) = input.files() {
                            for x in 0..file_list.length() {
                                if let Some(file) = file_list.get(x) {
                                    files.push(file.into());
                                }
                            }
                        }

                        Message::Files(files)
                    }) } />
                    <button
                        class={ yew::classes!("btn", "btn-outline-primary") }
                        type="button"
                        onclick={ ctx.link().callback(|_| Message::Import) }
                    >{ "Import" }</button>
                </div>
                <div class="input-group">
                    <a href={ export_url } target="_blank" class={ yew::classes!("btn", "btn-outline-primary") }>{ "Export" }</a>
                </div>
            </>
        }
    }

    crate::change!();
}
