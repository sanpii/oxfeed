update item
    set read = true
    from source
    join "user" using(user_id)
    where item.source_id = source.source_id
        and "user".token = $1
