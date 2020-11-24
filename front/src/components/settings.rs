#[derive(Clone)]
pub(crate) enum Message {
    Import,
    Imported,
    Loaded(Vec<u8>),
    Files(Vec<yew::web_sys::File>),
}

impl std::convert::TryFrom<(http::Method, yew::format::Text)> for Message {
    type Error = ();

    fn try_from((method, _): (http::Method, yew::format::Text)) -> Result<Self, ()> {
        let message = match method {
            http::Method::POST => Message::Imported,
            _ => return Err(()),
        };

        Ok(message)
    }
}

pub(crate) struct Component {
    event_bus: yew::agent::Dispatcher<crate::event::Bus>,
    fetch_task: Option<yew::services::fetch::FetchTask>,
    files: Vec<yew::web_sys::File>,
    link: yew::ComponentLink<Self>,
    tasks: Vec<yew::services::reader::ReaderTask>,
    reader: yew::services::reader::ReaderService,
}

impl Component {
    fn load(&mut self) {
        for file in &self.files {
            log::debug!("load {:?}", file);
            let callback = self.link.callback(|e: yew::services::reader::FileData| Message::Loaded(e.content));
            let task = self.reader.read_file(file.clone(), callback)
                .unwrap();
            self.tasks.push(task);
        }
    }

    fn import(&mut self, content: &[u8]) {
        let body = String::from_utf8(content.to_vec()).map_err(|err| anyhow::Error::new(err));
        self.fetch_task = crate::post(&self.link, "/opml", body).ok();
    }
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        use yew::agent::Dispatched;

        Self {
            event_bus: crate::event::Bus::dispatcher(),
            fetch_task: None,
            files: Vec::new(),
            link,
            tasks: Vec::new(),
            reader: yew::services::reader::ReaderService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Self::Message::Files(files) => self.files = files,
            Self::Message::Import => self.load(),
            Self::Message::Imported => {
                log::info!("Import done");
                self.event_bus.send(crate::event::Message::SettingUpdate);
                self.fetch_task = None;
            },
            Self::Message::Loaded(content) => self.import(&content),
        }

        true
    }

    fn view(&self) -> yew::Html {
        let export_url = format!("{}/opml", env!("API_URL"));
        let label = if self.files.is_empty() {
            "Choose a file".to_string()
        } else {
            self.files.iter()
                .map(|x| x.name())
                .collect::<Vec<_>>()
                .join(",")
        };

        yew::html! {
            <div class="card">
                <div class="card-header">{ "OPML" }</div>
                <div class="card-body">
                    <div class="input-group">
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" onchange=self.link.callback(|value| {
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
                            <label class="custom-file-label">{ label }</label>
                        </div>
                        <div class="input-group-append">
                            <button
                                class=("btn", "btn-outline-primary")
                                type="button"
                                onclick=self.link.callback(|_| Self::Message::Import)
                            >{ "Import" }</button>
                        </div>
                    </div>
                    <div class="input-group">
                        <a href=export_url class=("btn", "btn-outline-primary")>{ "Export" }</a>
                    </div>
                </div>
            </div>
        }
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        false
    }
}
