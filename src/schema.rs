table! {
    bles (id) {
        id -> Int4,
        title -> Varchar,
        idspace -> Int4,
    }
}

table! {
    coordinates (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        idspace -> Int4,
        idpoi -> Int4,
        blocked -> Bool,
        wallup -> Bool,
        wallright -> Bool,
        walldown -> Bool,
        wallleft -> Bool,
    }
}

table! {
    pois (id) {
        id -> Int4,
        title -> Varchar,
        idspace -> Int4,
        color -> Int8,
    }
}

table! {
    spaces (id) {
        id -> Int4,
        title -> Varchar,
        area -> Int4,
        longitude -> Float8,
        latitude -> Float8,
        dataset -> Varchar,
        compass -> Float8,
    }
}

joinable!(bles -> spaces (idspace));
joinable!(coordinates -> pois (idpoi));
joinable!(coordinates -> spaces (idspace));
joinable!(pois -> spaces (idspace));

allow_tables_to_appear_in_same_query!(
    bles,
    coordinates,
    pois,
    spaces,
);
