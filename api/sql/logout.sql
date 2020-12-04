update "user"
    set token = null
    where token = $1
