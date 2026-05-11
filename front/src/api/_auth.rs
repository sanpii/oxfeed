impl Api {
    pub async fn auth() -> oxfeed::Result<oxfeed::user::Entity> {
        Self::fetch(Method::GET, "/auth", ()).await
    }

    pub async fn auth_login(email: &str, password: &str, remember_me: &bool) -> oxfeed::Result {
        let headers = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let key = jsonwebtoken::EncodingKey::from_secret(
            env!("SECRET").as_bytes()
        );
        let mut claims = std::collections::BTreeMap::new();
        claims.insert("email", email);
        claims.insert("password", password);

        let token = jsonwebtoken::encode(&headers, &claims, &key)?;

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
