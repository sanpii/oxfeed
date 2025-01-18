/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/ws").service(websocket)
}

#[actix_web::get("")]
async fn websocket(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: actix_web::web::Query<crate::Identity>,
    request: actix_web::HttpRequest,
    body: actix_web::web::Payload,
) -> oxfeed::Result<actix_web::HttpResponse> {
    use futures_util::StreamExt as _;

    let token = identity.token(&elephantry)?;
    let (response, mut session, mut msg_stream) = actix_ws::handle(&request, body)?;

    let hb = std::sync::Arc::new(std::sync::Mutex::new(std::time::Instant::now()));

    {
        let mut session = session.clone();
        let hb = hb.clone();

        actix_web::rt::spawn(async move {
            let websocket = Websocket::new(elephantry.into_inner().get_default().unwrap(), token);
            let mut interval = actix_web::rt::time::interval(HEARTBEAT_INTERVAL);

            loop {
                interval.tick().await;

                if session.ping(b"").await.is_err() {
                    break;
                }

                if let Err(err) = websocket.notify(&mut session).await {
                    log::error!("{err}");
                }

                if std::time::Instant::now().duration_since(*hb.lock().unwrap()) > CLIENT_TIMEOUT {
                    session.close(None).await.ok();
                    break;
                }
            }
        });
    }

    {
        let hb = hb.clone();

        actix_web::rt::spawn(async move {
            while let Some(Ok(msg)) = msg_stream.next().await {
                match msg {
                    actix_ws::Message::Ping(bytes) => {
                        if session.pong(&bytes).await.is_err() {
                            return;
                        }
                    }
                    actix_ws::Message::Pong(_) => {
                        *hb.lock().unwrap() = std::time::Instant::now();
                    }
                    actix_ws::Message::Close(reason) => {
                        session.close(reason).await.ok();
                        return;
                    }
                    _ => (),
                }
            }
        });
    }

    Ok(response)
}

struct Websocket {
    elephantry: elephantry::Connection,
    token: uuid::Uuid,
}

impl Websocket {
    fn new(elephantry: &elephantry::Connection, token: uuid::Uuid) -> Self {
        let webosket = Self {
            elephantry: elephantry.clone(),
            token,
        };

        match webosket.elephantry.listen("item_new") {
            Ok(_) => (),
            Err(err) => log::error!("Unable to listen postgresql: {err}"),
        }

        webosket
    }

    async fn notify(&self, session: &mut actix_ws::Session) -> elephantry::Result {
        while let Some(notify) = self.elephantry.notifies()? {
            if notify.extra()? == self.token.to_string() {
                session.text(notify.relname()?).await.ok();
            }
        }

        Ok(())
    }
}

impl Drop for Websocket {
    fn drop(&mut self) {
        self.elephantry.unlisten("item_new").ok();
    }
}
