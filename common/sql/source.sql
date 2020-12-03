select source.*
    from source
    join "user" using(user_id)
    where source_id = $1
        and token = $2
