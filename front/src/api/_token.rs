impl Api {
    #[must_use]
    pub fn token() -> String {
        wasm_cookies::get("token")
            .unwrap_or_else(|| Ok(String::new()))
            .unwrap_or_default()
    }

    fn set_token(token: &str, remember_me: bool) {
        let expires = std::time::Duration::from_hours(365 * 24);
        let mut options = wasm_cookies::CookieOptions::default().expires_after(expires);

        if !remember_me {
            options.expires = None;
        }

        wasm_cookies::set("token", token, &options);
    }

    fn clear_token() {
        wasm_cookies::delete("token");
    }
}
