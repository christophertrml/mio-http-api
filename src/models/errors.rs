pub struct ApiError {
    pub code: String,
    pub message: String
}

impl ApiError {
    pub fn chat_already_exists(
        source_user_id: u64,
        destination_user_id: u64,
        existing_chat_id: u64) -> ApiError {
            ApiError {
                code: "chat_already_exists".to_string(),
                message: format!("A chat from user id '{}' to user id '{}' already exists, chat id: '{}' ", source_user_id, destination_user_id, existing_chat_id)
            }
    }
}