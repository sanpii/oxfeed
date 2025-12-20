select filter.*
    from filter
    join "user" using(user_id)
    where token = $1
    order by filter.name
