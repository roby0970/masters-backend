table! {
    coordinates (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        idpoi -> Nullable<Int4>,
    }
}

table! {
    pois (id) {
        id -> Int4,
        title -> Varchar,
        spaceid -> Nullable<Int4>,
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
joinable!(pois -> spaces (spaceid));

allow_tables_to_appear_in_same_query!(
    coordinates,
    pois,
    spaces,
);
