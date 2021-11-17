pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/auth")
        .service(login)
        .service(logout)
}

#[actix_web::post("/login")]
async fn login(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    token: actix_web::web::Json<String>,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let secret = crate::env("SECRET")?;

    use hmac::NewMac;
    use jwt::VerifyWithKey;

    let key: hmac::Hmac<sha2::Sha256> = hmac::Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let claims: std::collections::BTreeMap<String, String> = token.verify_with_key(&key)?;

    if claims.get("email").is_none() || claims.get("password").is_none() {
        return Ok(actix_web::HttpResponse::BadRequest().finish());
    }

    let sql = include_str!("../../sql/login.sql");
    let token = match elephantry
        .query::<uuid::Uuid>(sql, &[&claims["email"], &claims["password"]])?
        .try_get(0)
    {
        Some(token) => token,
        None => return Ok(actix_web::HttpResponse::Forbidden().finish()),
    };

    let response = actix_web::HttpResponse::Ok().json(&token.to_string());

    Ok(response)
}

#[actix_web::post("/logout")]
async fn logout(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let sql = include_str!("../../sql/logout.sql");
    elephantry.query::<()>(sql, &[&token])?;

    Ok(actix_web::HttpResponse::NoContent().finish())
}
