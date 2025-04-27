// @generated automatically by Diesel CLI.

diesel::table! {
    badges (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        date -> Timestamp,
        class -> Int2,
        tag_based -> Bool,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        score -> Nullable<Int4>,
        text -> Text,
        creation_date -> Timestamp,
        #[max_length = 255]
        user_display_name -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        content_license -> Nullable<Text>,
    }
}

diesel::table! {
    post_historys (id) {
        id -> Int4,
        post_history_type_id -> Int2,
        post_id -> Int4,
        revision_guid -> Nullable<Text>,
        creation_date -> Timestamp,
        user_id -> Nullable<Int4>,
        user_display_name -> Nullable<Text>,
        comment -> Nullable<Text>,
        text -> Nullable<Text>,
        content_license -> Nullable<Text>,
    }
}

diesel::table! {
    post_links (id) {
        id -> Int4,
        creation_date -> Timestamp,
        post_id -> Int4,
        related_post_id -> Int4,
        link_type_id -> Int2,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        post_type_id -> Int2,
        accepted_answer_id -> Nullable<Int4>,
        creation_date -> Timestamp,
        deletion_date -> Nullable<Timestamp>,
        score -> Nullable<Int4>,
        view_count -> Nullable<Int4>,
        body -> Nullable<Text>,
        owner_user_id -> Nullable<Int4>,
        owner_display_name -> Nullable<Text>,
        last_editor_user_id -> Nullable<Int4>,
        last_editor_display_name -> Nullable<Text>,
        last_edit_date -> Nullable<Timestamp>,
        last_activity_date -> Nullable<Timestamp>,
        title -> Nullable<Text>,
        tags -> Nullable<Text>,
        answer_count -> Nullable<Int4>,
        comment_count -> Nullable<Int4>,
        favorite_count -> Nullable<Int4>,
        close_date -> Nullable<Timestamp>,
        community_owned_date -> Nullable<Timestamp>,
        content_license -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        tag_name -> Text,
        count -> Int4,
        excerpt_post_id -> Nullable<Int4>,
        wiki_post_id -> Nullable<Int4>,
        is_moderator_only -> Int2,
        is_required -> Int2,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        reputation -> Int4,
        creation_date -> Timestamp,
        display_name -> Nullable<Text>,
        last_access_date -> Timestamp,
        website_url -> Nullable<Text>,
        location -> Nullable<Text>,
        about_me -> Nullable<Text>,
        views -> Nullable<Int4>,
        up_votes -> Nullable<Int4>,
        down_votes -> Nullable<Int4>,
        profile_image_url -> Nullable<Text>,
        email_hash -> Nullable<Text>,
        account_id -> Nullable<Int4>,
    }
}

diesel::table! {
    votes (id) {
        id -> Int4,
        post_id -> Int4,
        vote_type_id -> Int2,
        user_id -> Nullable<Int4>,
        creation_date -> Timestamp,
        bounty_amount -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    badges,
    comments,
    post_historys,
    post_links,
    posts,
    tags,
    users,
    votes,
);
