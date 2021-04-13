select i.title, i.item_id, i.link, i.published,  s.title as source, i.icon,
        i.read, i.favorite, s.tags
    from item i
    join source s using(source_id)
    join "user" using(user_id)
