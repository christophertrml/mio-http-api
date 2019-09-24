use std::collections::HashMap;
use crate::models::chat::Chat;
use crate::models::errors::ApiError;

// In-memory representation of database table with a single primary key index
pub struct ChatRepository {
    chat_index: HashMap<u64, Chat>,
    current_max_chat_id: u64
}

impl ChatRepository {
    pub fn new() -> ChatRepository {
        ChatRepository {
            chat_index: HashMap::new(),
            current_max_chat_id: 0
        }
    }

    pub fn insert(&mut self, source_user_id: u64, destination_user_id: u64) -> Result<Chat, ApiError> {
        match self.get_chat_id_if_exists(source_user_id, destination_user_id) {
            Some(chat_id) => Err(ApiError::chat_already_exists(source_user_id, destination_user_id, chat_id)),
            None => {
                self.current_max_chat_id += 1;
                let new_chat = Chat {
                    id: self.current_max_chat_id,
                    participant_ids: [source_user_id, destination_user_id]
                };
                self.chat_index.insert(self.current_max_chat_id, new_chat);
                Ok(new_chat)
            }
        }
    }

    fn get_chat_id_if_exists(&self, source_user_id: u64, destination_user_id: u64) -> Option<u64> {
        match self.chat_index.iter().find(|(_, chat)| 
                chat.participant_ids[0] == destination_user_id &&
                chat.participant_ids[1] == source_user_id) {
            Some((_, val)) => Some(val.id),
            None => None
        }
    }
}