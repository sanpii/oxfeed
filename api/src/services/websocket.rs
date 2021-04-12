use actix::ActorContext;
use actix_web_actors::ws;

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
    stream: actix_web::web::Payload,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let websocket = Websocket::new(elephantry.into_inner(), token);
    let response = ws::start(websocket, &request, stream)?;

    Ok(response)
}

struct Websocket {
    elephantry: std::sync::Arc<elephantry::Pool>,
    hb: std::time::Instant,
    token: uuid::Uuid,
}

impl Websocket {
    fn new(elephantry: std::sync::Arc<elephantry::Pool>, token: uuid::Uuid) -> Self {
        Self {
            elephantry,
            hb: std::time::Instant::now(),
            token,
        }
    }

    fn hb(&self, context: &mut <Self as actix::Actor>::Context) {
        use actix::AsyncContext;

        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            actor.ping(context);
            if let Err(err) = actor.notify(context) {
                log::error!("{}", err);
            }
        });
    }

    fn ping(&self, context: &mut <Self as actix::Actor>::Context) {
        if std::time::Instant::now().duration_since(self.hb) > CLIENT_TIMEOUT {
            log::warn!("Websocket Client heartbeat failed, disconnecting!");
            context.stop();
            return;
        }

        context.ping(b"");
    }

    fn notify(&self, context: &mut <Self as actix::Actor>::Context) -> elephantry::Result {
        while let Some(notify) = self.elephantry.notifies()? {
            if notify.extra() == self.token.to_string() {
                context.text(notify.relname());
            }
        }

        Ok(())
    }
}

impl actix::Actor for Websocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        match self.elephantry.listen("item_new") {
            Ok(_) => (),
            Err(err) => log::error!("Unable to listen postgresql: {}", err),
        }

        self.hb(context);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.elephantry.unlisten("item_new").ok();
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, context: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = std::time::Instant::now();
                context.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => self.hb = std::time::Instant::now(),
            Ok(ws::Message::Close(reason)) => {
                context.close(reason);
                context.stop();
            }
            _ => context.stop(),
        }
    }
}
