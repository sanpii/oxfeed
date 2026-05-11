update "user"
    set token = uuidv4()
    where email = $1 and password = crypt($2, password)
    returning token
