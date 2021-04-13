pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/tags")
        .service(all)
}

#[actix_web::get("")]
async fn all(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    pagination: actix_web::web::Query<oxfeed_common::Pagination>,
    identity: crate::Identity,
) -> oxfeed_common::Result<actix_web::HttpResponse> {
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

    let rows = elephantry.query::<oxfeed_common::Tag>(&query, &params)?;

    let response = actix_web::HttpResponse::Ok().json(rows);

    Ok(response)
}
