table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        discriminator -> Integer,
        unixCreationTime -> Integer,
    }
}
