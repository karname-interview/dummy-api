table! {
    tours (id) {
        id -> Uuid,
        name -> Varchar,
        src -> Varchar,
        dst -> Varchar,
        total_days -> Int4,
    }
}
