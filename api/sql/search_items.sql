select ts_headline(i.title, websearch_to_tsquery($2), 'StartSel = <mark>, StopSel = </mark>') as title,
        i.item_id, i.link, i.published,  s.title as source, i.icon, i.read,
        i.favorite, s.tags
    from fts.item f
    join item i using(item_id)
    join source s using(source_id)
    join "user" using(user_id)
    where token = $1
        and document @@ websearch_to_tsquery($2)
