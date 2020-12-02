select content
    from item
    join source using(source_id)
    join "user" using(user_id)
    where item_id = $1
        and token = $2
