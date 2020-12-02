update "user"
    set token = uuid_generate_v4()
    where (email = $1 or name = $1) and password = crypt($2, password)
    returning token
