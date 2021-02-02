with items as (
    select i.item_id, i.link, i.published, i.title, s.title as source, i.icon, i.read, i.favorite, s.tags
        from fts.item
        join item i using(item_id)
        join source s using(source_id)
        join "user" using(user_id)
        where token = $1
            and document @@ to_tsquery($2)
)
select count(items)
    from items
