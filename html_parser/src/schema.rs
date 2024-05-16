// @generated automatically by Diesel CLI.

diesel::table! {
    element_selector (id) {
        id -> Uuid,
        element_id -> Uuid,
        selector_id -> Uuid,
        matches -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    elements (id) {
        id -> Uuid,
        page_id -> Uuid,
        name -> Nullable<Varchar>,
        html -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pages (id) {
        id -> Uuid,
        name -> Varchar,
        url -> Varchar,
        html -> Text,
        created_at -> Nullable<Timestamp>,
        html_hash -> Varchar,
    }
}

diesel::table! {
    selectors (id) {
        id -> Uuid,
        path -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(element_selector -> elements (element_id));
diesel::joinable!(element_selector -> selectors (selector_id));
diesel::joinable!(elements -> pages (page_id));

diesel::allow_tables_to_appear_in_same_query!(
    element_selector,
    elements,
    pages,
    selectors,
);
