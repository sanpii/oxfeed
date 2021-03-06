#[derive(Clone)]
pub(crate) enum Message {
    Import,
    Imported,
    Loaded(Vec<u8>),
    Files(Vec<yew::web_sys::File>),
}

impl From<crate::event::Api> for Message {
    fn from(event: crate::event::Api) -> Self {
        match event {
            crate::event::Api::OpmlImport => Self::Imported,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct Component {
    api: crate::Api<Self>,
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
            let task = yew::services::reader::ReaderService::read_file(file.clone(), callback).unwrap();
            self.tasks.push(task);
        }
    }

    fn import(&mut self, content: &[u8]) {
        let opml = String::from_utf8(content.to_vec()).map_err(anyhow::Error::new);
        self.api.opml_import(opml);
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            api: crate::Api::new(link.clone()),
            event_bus: crate::event::Bus::dispatcher(),
            files: Vec::new(),
            link,
            tasks: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Files(files) => self.files = files,
            Self::Message::Import => self.load(),
            Self::Message::Imported => {
                let alert = crate::event::Alert::info("Import done");
                self.event_bus.send(crate::event::Event::Alert(alert));
                self.event_bus.send(crate::event::Event::SettingUpdate);
            }
            Self::Message::Loaded(content) => self.import(&content),
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

                        Self::Message::Files(files)
                    }) />
                    <button
                        class=yew::classes!("btn", "btn-outline-primary")
                        type="button"
                        onclick=self.link.callback(|_| Self::Message::Import)
                    >{ "Import" }</button>
                </div>
                <div class="input-group">
                    <a href=export_url class=yew::classes!("btn", "btn-outline-primary")>{ "Export" }</a>
                </div>
            </>
        }
    }

    crate::change!();
}
