select ts_headline(i.title, websearch_to_tsquery($1), 'StartSel = <mark>, StopSel = </mark>') as title,
        i.item_id, i.link, i.published, s.title as source, s.icon, i.read,
        i.favorite, s.tags, array_remove(array_agg(media), null) as media
    from fts.item f
    join item i using(item_id)
    join source s using(source_id)
    left join media using(item_id)
    join "user" using(user_id)
