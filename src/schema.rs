table! {
    coordinates (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        idspace -> Int4,
        idpoi -> Int4,
        blocked -> Bool,
    }
}

table! {
    pois (id) {
        id -> Int4,
        title -> Varchar,
        idspace -> Int4,
    }
}

table! {
    spaces (id) {
        id -> Int4,
        title -> Varchar,
        area -> Int4,
        longitude -> Float8,
        latitude -> Float8,
    }
}

joinable!(coordinates -> pois (idpoi));
joinable!(coordinates -> spaces (idspace));
joinable!(pois -> spaces (idspace));

allow_tables_to_appear_in_same_query!(
    coordinates,
    pois,
    spaces,
);
