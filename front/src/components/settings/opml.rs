pub(crate) enum Message {
    Error(oxfeed_common::Error),
    Import,
    Imported,
    Loaded(Vec<u8>),
    Files(Vec<yew::web_sys::File>),
}

pub(crate) struct Component {
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    files: Vec<yew::web_sys::File>,
    link: yew::ComponentLink<Self>,
    tasks: Vec<yew::services::reader::ReaderTask>,
}

impl Component {
    fn load(&mut self) {
        for file in &self.files {
            let callback = self
                .link
                .callback(|e: yew::services::reader::FileData| Message::Loaded(e.content));
            let task =
                yew::services::reader::ReaderService::read_file(file.clone(), callback).unwrap();
            self.tasks.push(task);
        }
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            files: Vec::new(),
            link,
            tasks: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::Error(err) => self.event_bus.send(err.into()),
            Message::Files(files) => self.files = files,
            Message::Import => self.load(),
            Message::Imported => {
                let alert = crate::event::Alert::info("Import done");
                self.event_bus.send(crate::Event::Alert(alert));
                self.event_bus.send(crate::Event::SettingUpdate);
            }
            Message::Loaded(content) => {
                use yewtil::future::LinkFuture;

                self.link.send_future(async move {
                    let opml = String::from_utf8(content.to_vec()).map_err(anyhow::Error::new);
                    crate::Api::opml_import(opml)
                        .await
                        .map_or_else(Message::Error, |_| Message::Imported)
                });
            }
        }

        true
    }

    fn view(&self) -> yew::Html {
        let export_url = format!("{}/opml", env!("API_URL"));

        yew::html! {
            <>
                <div class="input-group mb-3">
                    <input type="file" class="form-control" onchange=self.link.callback(|value| {
                        let mut files = Vec::new();

                        if let yew::ChangeData::Files(file_list) = value {
                            for x in 0..file_list.length() {
                                if let Some(file) = file_list.get(x) {
                                    files.push(file);
                                }
                            }
                        }

                        Message::Files(files)
                    }) />
                    <button
                        class=yew::classes!("btn", "btn-outline-primary")
                        type="button"
                        onclick=self.link.callback(|_| Message::Import)
                    >{ "Import" }</button>
                </div>
                <div class="input-group">
                    <a href=export_url target="_blank" class=yew::classes!("btn", "btn-outline-primary")>{ "Export" }</a>
                </div>
            </>
        }
    }

    crate::change!();
}
