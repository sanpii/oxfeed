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
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let user = elephantry
        .model::<oxfeed::user::Model>()
        .find_from_token(&token);

    let response = actix_web::HttpResponse::Ok().json(&user);

    Ok(response)
}

#[actix_web::post("/login")]
async fn login(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    token: actix_web::web::Json<String>,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let secret = envir::get("SECRET")?;
    let key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims::<&str>(&[]);

    let data = jsonwebtoken::decode::<std::collections::BTreeMap<String, String>>(
        token.as_bytes(),
        &key,
        &validation,
    )?;

    if !data.claims.contains_key("email") || !data.claims.contains_key("password") {
        return Err(oxfeed::Error::BadRequest);
    }

    let sql = include_str!("../../sql/login.sql");
    let token = elephantry
        .query::<uuid::Uuid>(sql, &[&data.claims["email"], &data.claims["password"]])?
        .try_get(0)
        .ok_or(oxfeed::Error::InvalidLogin)?;

    let response = actix_web::HttpResponse::Ok().json(token.to_string());

    Ok(response)
}

#[actix_web::post("/logout")]
async fn logout(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;
    let sql = include_str!("../../sql/logout.sql");
    elephantry.query::<()>(sql, &[&token])?;

    Ok(actix_web::HttpResponse::NoContent().finish())
}
