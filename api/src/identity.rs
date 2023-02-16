#[derive(Default, serde::Deserialize)]
pub(crate) struct Identity {
    token: uuid::Uuid,
}

impl Identity {
    pub fn token(&self, elephantry: &elephantry::Connection) -> oxfeed_common::Result<uuid::Uuid> {
        use oxfeed_common::user::Model;

        if elephantry.exist_where::<Model>("token = $*", &[&self.token])? {
            Ok(self.token)
        } else {
            Err(oxfeed_common::Error::Auth)
        }
    }

    fn unauthorized() -> futures_util::future::Ready<oxfeed_common::Result<Self>> {
        futures_util::future::err(oxfeed_common::Error::Auth)
    }
}

impl actix_web::FromRequest for Identity {
    type Error = oxfeed_common::Error;
    type Future = futures_util::future::Ready<oxfeed_common::Result<Self>>;

    #[inline]
    fn from_request(
        request: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let Some(authorization) = request
            .headers()
            .get("Authorization")
            .and_then(|x| x.to_str().ok()) else {
            return Self::unauthorized();
        };

        let Some(mid) = authorization.find(' ') else {
            return Self::unauthorized();
        };

        let (ty, token) = authorization.split_at(mid);

        if ty.eq_ignore_ascii_case("bearer") {
            match token.trim().parse() {
                Ok(token) => futures_util::future::ok(Identity { token }),
                _ => Self::unauthorized(),
            }
        } else {
            Self::unauthorized()
        }
    }
}
