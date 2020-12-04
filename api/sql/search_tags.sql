with tags as (
    select unnest(tags) as tag
        from source
        join "user" using (user_id)
        where token = $1
)
select distinct tag
    from tags
    where tag ~* $2
    order by 1
