// @generated automatically by Diesel CLI.

diesel::table! {
    private_keys (key_id) {
        key_id -> Text,
        nickname -> Text,
        private_key -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    public_keys (key_id) {
        key_id -> Text,
        nickname -> Text,
        public_key -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    private_keys,
    public_keys,
);
