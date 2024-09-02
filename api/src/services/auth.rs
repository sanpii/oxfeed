pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/auth")
        .service(get)
        .service(login)
        .service(logout)
}

#[actix_web::get("")]
async fn get(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let user = elephantry
        .model::<oxfeed_common::user::Model>()
        .find_from_token(&token);

    let response = actix_web::HttpResponse::Ok().json(&user);

    Ok(response)
}

#[actix_web::post("/login")]
async fn login(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    token: actix_web::web::Json<String>,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
    use hmac::Mac;
    use jwt::VerifyWithKey;

    let secret = envir::get("SECRET")?;
    let key: hmac::Hmac<sha2::Sha256> = hmac::Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let claims: std::collections::BTreeMap<String, String> = token.verify_with_key(&key)?;

    if !claims.contains_key("email") || !claims.contains_key("password") {
        return Err(oxfeed_common::Error::BadRequest);
    }

    let sql = include_str!("../../sql/login.sql");
    let token = elephantry
        .query::<uuid::Uuid>(sql, &[&claims["email"], &claims["password"]])?
        .try_get(0)
        .ok_or(oxfeed_common::Error::InvalidLogin)?;

    let response = actix_web::HttpResponse::Ok().json(token.to_string());

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
