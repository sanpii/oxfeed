#[derive(Default, Debug, serde::Deserialize)]
pub struct Identity {
    token: Option<uuid::Uuid>,
}

impl Identity {
    pub fn token(&self) -> Option<uuid::Uuid> {
        self.token
    }
}

impl actix_web::FromRequest for Identity {
    type Config = ();
    type Error = oxfeed_common::Error;
    type Future = futures_util::future::Ready<oxfeed_common::Result<Self>>;

    #[inline]
    fn from_request(
        request: &actix_web::web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let authorization = match request
            .headers()
            .get("Authorization")
            .map(|x| x.to_str().ok())
            .flatten()
        {
            Some(authorization) => authorization,
            None => return futures_util::future::ok(Self::default()),
        };

        let mid = match authorization.find(' ') {
            Some(mid) => mid,
            None => return futures_util::future::ok(Self::default()),
        };

        let (ty, token) = authorization.split_at(mid);

        let token = if ty.eq_ignore_ascii_case("bearer") {
            match token.trim().parse() {
                Ok(token) => Some(token),
                Err(_) => None,
            }
        } else {
            None
        };

        futures_util::future::ok(Identity { token })
    }
}
