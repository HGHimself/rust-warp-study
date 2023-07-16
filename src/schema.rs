table! {
    background (id) {
        id -> Int4,
        count -> Int4,
        frequency -> Int4,
        x_amplitude -> Int4,
        y_amplitude -> Int4,
        x_multiplier -> Int4,
        y_multiplier -> Int4,
        color -> Int4,
        thickness -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    link (id) {
        id -> Int4,
        url -> Varchar,
        creator_user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        img_url -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
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
        background_id -> Int4,
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
        background_id -> Int4,
    }
}

joinable!(link -> user (creator_user_id));
joinable!(page -> background (background_id));
joinable!(page_link -> link (link_id));
joinable!(page_link -> page (page_id));
joinable!(user -> background (background_id));

allow_tables_to_appear_in_same_query!(
    background,
    link,
    page,
    page_link,
    session,
    user,
);
