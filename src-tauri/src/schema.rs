// @generated automatically by Diesel CLI.

diesel::table! {
    keys (key_id) {
        key_id -> Text,
        nickname -> Text,
        metadata -> Nullable<Text>,
        is_me -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        created_at -> Timestamp,
    }
}
