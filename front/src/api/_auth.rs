impl Api {
    pub async fn auth() -> oxfeed::Result<oxfeed::user::Entity> {
        Self::fetch(Method::GET, "/auth", ()).await
    }

    pub async fn auth_login(email: &str, password: &str, remember_me: &bool) -> oxfeed::Result {
        use hmac::Mac;
        use jwt::SignWithKey;

        let key: hmac::Hmac<sha2::Sha256> =
            hmac::Hmac::new_from_slice(env!("SECRET").as_bytes()).unwrap();
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("email", email);
        claims.insert("password", password);

        let token = claims.sign_with_key(&key).unwrap();

        let data: String = Self::fetch(Method::POST, "/auth/login", token).await?;

        Self::set_token(&data, *remember_me);

        Ok(())
    }

    pub async fn auth_logout() -> oxfeed::Result {
        Self::fetch::<_, ()>(Method::POST, "/auth/logout", ()).await?;

        Self::clear_token();

        Ok(())
    }
}
