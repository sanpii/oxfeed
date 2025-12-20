with deleted_filter as(
    delete from filter f
        using "user" u
        where u.user_id = f.user_id
            and filter_id = $1
            and token = $2
        returning f.*
)
update source s
    set filters = array_remove(filters, f.filter_id)
    from deleted_filter f
    where f.user_id = s.user_id
    returning f.*
