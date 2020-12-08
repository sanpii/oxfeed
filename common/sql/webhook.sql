select webhook.*
    from webhook
    join "user" using(user_id)
    where webhook_id = $1
        and token = $2
