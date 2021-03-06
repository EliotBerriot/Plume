table! {
    blog_authors (id) {
        id -> Int4,
        blog_id -> Int4,
        author_id -> Int4,
        is_owner -> Bool,
    }
}

table! {
    blogs (id) {
        id -> Int4,
        actor_id -> Varchar,
        title -> Varchar,
        summary -> Text,
        outbox_url -> Varchar,
        inbox_url -> Varchar,
        instance_id -> Int4,
        creation_date -> Timestamp,
        ap_url -> Text,
        private_key -> Nullable<Text>,
        public_key -> Text,
    }
}

table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        in_response_to_id -> Nullable<Int4>,
        post_id -> Int4,
        author_id -> Int4,
        creation_date -> Timestamp,
        ap_url -> Nullable<Varchar>,
        sensitive -> Bool,
        spoiler_text -> Text,
    }
}

table! {
    follows (id) {
        id -> Int4,
        follower_id -> Int4,
        following_id -> Int4,
    }
}

table! {
    instances (id) {
        id -> Int4,
        public_domain -> Varchar,
        name -> Varchar,
        local -> Bool,
        blocked -> Bool,
        creation_date -> Timestamp,
        open_registrations -> Bool,
        short_description -> Text,
        long_description -> Text,
        default_license -> Text,
        long_description_html -> Varchar,
        short_description_html -> Varchar,
    }
}

table! {
    likes (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        creation_date -> Timestamp,
        ap_url -> Varchar,
    }
}

table! {
    medias (id) {
        id -> Int4,
        file_path -> Text,
        alt_text -> Text,
        is_remote -> Bool,
        remote_url -> Nullable<Text>,
        sensitive -> Bool,
        content_warning -> Nullable<Text>,
        owner_id -> Int4,
    }
}

table! {
    mentions (id) {
        id -> Int4,
        mentioned_id -> Int4,
        post_id -> Nullable<Int4>,
        comment_id -> Nullable<Int4>,
        ap_url -> Varchar,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        creation_date -> Timestamp,
        kind -> Varchar,
        object_id -> Int4,
    }
}

table! {
    post_authors (id) {
        id -> Int4,
        post_id -> Int4,
        author_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        blog_id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        license -> Varchar,
        creation_date -> Timestamp,
        ap_url -> Varchar,
    }
}

table! {
    reshares (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        ap_url -> Varchar,
        creation_date -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        display_name -> Varchar,
        outbox_url -> Varchar,
        inbox_url -> Varchar,
        is_admin -> Bool,
        summary -> Text,
        email -> Nullable<Text>,
        hashed_password -> Nullable<Text>,
        instance_id -> Int4,
        creation_date -> Timestamp,
        ap_url -> Text,
        private_key -> Nullable<Text>,
        public_key -> Text,
        shared_inbox_url -> Nullable<Varchar>,
        followers_endpoint -> Varchar,
        avatar_id -> Nullable<Int4>,
    }
}

joinable!(blog_authors -> blogs (blog_id));
joinable!(blog_authors -> users (author_id));
joinable!(blogs -> instances (instance_id));
joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(likes -> posts (post_id));
joinable!(likes -> users (user_id));
joinable!(mentions -> comments (comment_id));
joinable!(mentions -> posts (post_id));
joinable!(mentions -> users (mentioned_id));
joinable!(notifications -> users (user_id));
joinable!(post_authors -> posts (post_id));
joinable!(post_authors -> users (author_id));
joinable!(posts -> blogs (blog_id));
joinable!(reshares -> posts (post_id));
joinable!(reshares -> users (user_id));
joinable!(users -> instances (instance_id));

allow_tables_to_appear_in_same_query!(
    blog_authors,
    blogs,
    comments,
    follows,
    instances,
    likes,
    medias,
    mentions,
    notifications,
    post_authors,
    posts,
    reshares,
    users,
);
