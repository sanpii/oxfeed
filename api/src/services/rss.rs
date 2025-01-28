use oxfeed::item::Model;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/rss").service(index)
}

#[actix_web::get("/{token}")]
async fn index(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    user_id: actix_web::web::Path<uuid::Uuid>,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let model = elephantry.model::<Model>();
    let items = model
        .rss(&user_id)?
        .into_iter()
        .map(|x| {
            let categories = x
                .tags
                .iter()
                .map(|tag| format!("<category>{}</category>", html_encode(tag)))
                .collect::<Vec<_>>()
                .join("\n");

            let enclosures = x
                .media
                .iter()
                .map(|media| {
                    format!(
                        "<enclosure url=\"{}\" type=\"{}\" />",
                        media.url,
                        media.content_type.clone().unwrap_or_default()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                r#"
        <item>
            <guid isPermaLink="false">{guid}</guid>
            <title>{title}</title>
            <description><![CDATA[{description}]]></description>
            <link>{link}</link>
            <pubDate>{pubdate}</pubDate>
{categories}
{enclosures}
        </item>
"#,
                guid = x.id,
                title = x.title,
                description = x.content.unwrap_or_default(),
                link = x.link,
                pubdate = x.published.to_rfc2822()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let rss = format!(
        r#"
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>My oxfeed subscriptions</title>
        <generator>oxfeed</generator>
        {items}
    </channel>
</rss>
"#
    );

    let response = actix_web::HttpResponse::Ok()
        .content_type(mime::TEXT_XML)
        .body(rss);

    Ok(response)
}

fn html_encode(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
