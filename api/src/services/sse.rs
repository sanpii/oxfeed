const HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/sse").service(index)
}

#[actix_web::get("")]
async fn index(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: actix_web::web::Query<crate::Identity>,
    broadcaster: actix_web::web::Data<std::sync::Arc<Broadcaster>>,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let client = broadcaster.new_client(&token);

    let response = actix_web::HttpResponse::Ok()
        .content_type(mime::TEXT_EVENT_STREAM)
        .streaming(client);

    Ok(response)
}

pub(crate) struct Broadcaster {
    elephantry: elephantry::Connection,
    clients: std::sync::Mutex<Vec<std::sync::mpsc::Sender<(String, String)>>>,
}

impl Broadcaster {
    pub fn new(elephantry: &elephantry::Connection) -> std::sync::Arc<Self> {
        let broadcaster = Self {
            elephantry: elephantry.clone(),
            clients: std::sync::Mutex::default(),
        };

        let arc = std::sync::Arc::new(broadcaster);

        Self::listen(std::sync::Arc::clone(&arc));

        arc
    }

    fn listen(this: std::sync::Arc<Self>) {
        std::thread::spawn(move || {
            if let Err(err) = this.elephantry.listen("item_new") {
                log::error!("Unable to listen postgresql: {err}");
            }

            let wait = std::time::Duration::new(1, 0);

            loop {
                if let Ok(Some(notify)) = this.elephantry.notifies() {
                    this.broadcast((
                        notify.relname().unwrap().to_string(),
                        notify.extra().unwrap(),
                    ));
                }
                std::thread::sleep(wait);
            }
        });
    }

    fn broadcast(&self, message: (String, String)) {
        self.clients.lock().unwrap().iter().for_each(move |c| {
            c.send(message.clone()).ok();
        });
    }

    pub fn new_client(&self, token: &uuid::Uuid) -> Client {
        let (tx, rx) = std::sync::mpsc::channel();

        self.clients.lock().unwrap().push(tx);

        Client {
            rx,
            token: *token,
            keep_alive: actix::clock::interval(HEARTBEAT_INTERVAL),
        }
    }
}

pub(crate) struct Client {
    rx: std::sync::mpsc::Receiver<(String, String)>,
    token: uuid::Uuid,
    keep_alive: actix_web::rt::time::Interval,
}

impl futures_util::Stream for Client {
    type Item = oxfeed::Result<actix_web::web::Bytes>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use std::task::Poll::*;

        let client = self.get_mut();

        if let Ok(message) = client.rx.try_recv()
            && message.1 == client.token.to_string()
        {
            return Event::Data(message.0).into();
        }

        if client.keep_alive.poll_tick(cx).is_ready() {
            return Event::KeepAlive.into();
        }

        Pending
    }
}

enum Event {
    Data(String),
    KeepAlive,
}

impl Event {
    fn into_bytes(self) -> actix_web::web::Bytes {
        match self {
            Self::Data(message) => format!("data: {message}\n\n").into(),
            Self::KeepAlive => actix_web::web::Bytes::from_static(b": keep-alive\n\n"),
        }
    }
}

impl From<Event> for std::task::Poll<Option<oxfeed::Result<actix_web::web::Bytes>>> {
    fn from(value: Event) -> Self {
        std::task::Poll::Ready(Some(Ok(value.into_bytes())))
    }
}
