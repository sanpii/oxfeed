select distinct unnest(tags)
    from source
    join "user" using(user_id)
    where "user".token = $*
