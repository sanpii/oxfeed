update source s
    set tags=array_replace(tags, $1, $2)
    from "user" u
    where s.user_id = u.user_id
        and u.token = $3
        and $1 = any(s.tags)
