use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

// Define the tables
table! {
    members (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    library (id) {
        id -> Integer,
        name -> Text,
        address -> Text,
        created_at -> Text,
        updated_at -> Text,
        manager_id -> Integer,
    }
}

table! {
    books (id) {
        id -> Integer,
        title -> Text,
        author -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

// Define the many-to-many relationship tables
table! {
    library_books (library_id, book_id) {
        library_id -> Integer,
        book_id -> Integer,
        quantity -> Integer,
    }
}

table! {
    library_members (library_id, member_id) {
        library_id -> Integer,
        member_id -> Integer,
    }
}

table! {
    borrowed_books (member_id, book_id) {
        member_id -> Integer,
        book_id -> Integer,
        borrowed_at -> Text,
    }
}

// Define the relationships
joinable!(books -> library (id));
joinable!(library -> members (manager_id));
joinable!(library_books -> library (library_id));
joinable!(library_books -> books (book_id));
joinable!(library_members -> library (library_id));
joinable!(library_members -> members (member_id));
joinable!(borrowed_books -> members (member_id));
joinable!(borrowed_books -> books (book_id));

// Allow tables to appear in the same query
allow_tables_to_appear_in_same_query!(
    members,
    library,
    books,
    library_books,
    library_members,
);