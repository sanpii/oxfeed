use oxfeed::item::Model;

pub(crate) fn scope() -> actix_web::Scope {
    actix_web::web::scope("/rss").service(index)
}

#[derive(serde::Serialize)]
struct Context {
    items: Vec<oxfeed::item::FeedItem>,
}

static TEMPLATE: &str = r#"
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>My oxfeed subscriptions</title>
        <generator>oxfeed</generator>
        {{ for item in items }}
            <item>
                <guid isPermaLink="false">{ item.id }</guid>
                <title>{ item.title }</title>
                {{ if item.content }}
                    <description><![CDATA[{ item.content | unescaped }]]></description>
                {{ endif }}
                <link>{ item.link }</link>
                <pubDate>{ item.published }</pubDate>
                {{ for tag in item.tags }}
                    <category>{ tag }</category>
                {{ endfor }}
                {{ for media in item.media }}
                    <enclosure url="{ media.url }" type="{ media.content_type }" />
                {{ endfor }}
            </item>
        {{ endfor }}
    </channel>
</rss>
"#;

#[actix_web::get("/{token}")]
async fn index(
    elephantry: actix_web::web::Data<elephantry::Pool>,
    user_id: actix_web::web::Path<uuid::Uuid>,
) -> oxfeed::Result<actix_web::HttpResponse> {
    let model = elephantry.model::<Model>();
    let items = model.rss(&user_id)?;
    let mut template = tinytemplate::TinyTemplate::new();
    template.add_template("rss", TEMPLATE).unwrap();

    let context = Context { items };
    let rss = template.render("rss", &context).unwrap();

    let response = actix_web::HttpResponse::Ok()
        .content_type(mime::TEXT_XML)
        .body(rss);

    Ok(response)
}
