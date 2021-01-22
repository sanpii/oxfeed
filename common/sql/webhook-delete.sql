with deleted_webhook as(
    delete from webhook w
        using "user" u
        where u.user_id = w.user_id
            and webhook_id = $1
            and token = $2
        returning w.*
)
update source s
    set webhooks = array_remove(webhooks, w.webhook_id)
    from deleted_webhook w
    where w.user_id = s.user_id
