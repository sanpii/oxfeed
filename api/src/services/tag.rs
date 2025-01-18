pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/tags").service(all).service(rename)
}

#[actix_web::get("")]
async fn all(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    pagination: actix_web::web::Query<elephantry_extras::Pagination>,
    identity: crate::Identity,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;

    let mut clause = elephantry::Where::new();
    clause.and_where("\"user\".token = $*", vec![&token]);
    let params = clause.params();

    let query = format!(
        r#"
select unnest(tags) as name, count(*) as count
    from source
    join "user" using (user_id)
    where {}
    group by name
    order by name
    {}
        "#,
        clause.to_string(),
        pagination.to_sql(),
    );

    let rows = elephantry.query::<oxfeed::Tag>(&query, &params)?;

    let response = actix_web::HttpResponse::Ok().json(rows);

    Ok(response)
}

#[actix_web::post("/{tag}")]
async fn rename(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    identity: crate::Identity,
    tag: actix_web::web::Path<String>,
    actix_web::web::Json(name): actix_web::web::Json<String>,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let token = identity.token(&elephantry)?;

    let query = include_str!("../../sql/rename_tag.sql");

    elephantry.query::<oxfeed::Tag>(query, &[&tag.into_inner(), &name, &token])?;

    Ok(actix_web::HttpResponse::NoContent().finish())
}
