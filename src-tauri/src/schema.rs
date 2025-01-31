// @generated automatically by Diesel CLI.

diesel::table! {
    private_keys (key_id) {
        key_id -> Text,
        nickname -> Text,
        metadata -> Nullable<Text>,
        private_key -> Text,
        public_key_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    public_keys (key_id) {
        key_id -> Text,
        nickname -> Text,
        metadata -> Nullable<Text>,
        is_me -> Bool,
        public_key -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(private_keys -> public_keys (public_key_id));

diesel::allow_tables_to_appear_in_same_query!(
    private_keys,
    public_keys,
);
