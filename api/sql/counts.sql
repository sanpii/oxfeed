with
    count_sources as (
        select count(*), count(*) filter (where last_error is not null) > 0 errors
            from source
            join "user" using(user_id)
            where "user".token = $1
    ),
    user_tags as (
        select distinct unnest(tags)
            from source
            join "user" using(user_id)
            where "user".token = $1
    ),
    count_tags as (
        select count(*) from user_tags
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
        count_tags.count as tags,
        count_unread.count as unread,
        count_all.count as all,
        count_favorites.count as favorites,
        count_sources.errors as sources_has_error
    from count_sources,
        count_tags,
        count_unread,
        count_all,
        count_favorites
