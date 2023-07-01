table! {
    link (id) {
        id -> Int4,
        url -> Varchar,
        creator_user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    page (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    page_link (id) {
        id -> Int4,
        link_id -> Int4,
        page_id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    session (id) {
        id -> Int4,
        user_id -> Int4,
        valid_until -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(link -> user (creator_user_id));
joinable!(page_link -> link (link_id));
joinable!(page_link -> page (page_id));

allow_tables_to_appear_in_same_query!(link, page, page_link, session, user,);
