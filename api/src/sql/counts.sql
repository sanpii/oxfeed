with
    count_sources as (
        select count(*)
            from source
            join "user" using(user_id)
            where "user".token = $1
    ),
    user_item as (
        select *
            from item
            join source using(source_id)
            join "user" using(user_id)
            where "user".token = $1
    ),
    count_unread as (
        select count(*) from user_item where not read
    ),
    count_all as (
        select count(*) from user_item
    ),
    count_favorites as (
        select count(*) from user_item where favorite
    )
select count_sources.count as sources,
        count_unread.count as unread,
        count_all.count as all,
        count_favorites.count as favorites
    from count_sources,
        count_unread,
        count_all,
        count_favorites
