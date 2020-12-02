impl<C> super::Api<C> where C: yew::Component, <C as yew::Component>::Message: From<crate::event::Api> {
    pub fn auth_login(&mut self, login: &str, password: &str) {
        use hmac::NewMac;
        use jwt::SignWithKey;

        let key: hmac::Hmac<sha2::Sha256> = hmac::Hmac::new_varkey(env!("SECRET").as_bytes()).unwrap();
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("login", login);
        claims.insert("password", password);

        let token = claims.sign_with_key(&key).unwrap();

        self.fetch(super::Kind::AuthLogin, http::Method::POST, "/auth/login", yew::format::Json(&token))
    }
}
