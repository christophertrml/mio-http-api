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
            Some(chat) => Err(ApiError::chat_already_exists(source_user_id, destination_user_id, chat.id)),
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

    pub fn get_chats_for_user(&self, user_id: u64) -> Vec<&Chat> {
        self.chat_index.iter().filter_map(|(_, chat)| {
            if chat.participant_ids[0] == user_id
                || chat.participant_ids[1] == user_id {
                    Some(chat)
                }
                else {
                    None
                }
        }).collect()
    }

    fn get_chat_id_if_exists(&self, source_user_id: u64, destination_user_id: u64) -> Option<&Chat> {
        match self.chat_index.iter().find(|(_, chat)| 
                chat.participant_ids[0] == source_user_id &&
                chat.participant_ids[1] == destination_user_id) {
            Some((_, val)) => Some(val),
            None => None
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    #[test]
    fn insert_errors_on_attempted_reinsert() {
        let mut repository = ChatRepository::new();
        repository.insert(1, 2);
        repository.insert(2, 3);
        repository.insert(3, 1);
        let actual = repository.insert(2, 3);
        assert!(actual.is_err());
        match actual {
            Ok(_) => {
                assert!(false);
            }
            Err(err) => {
                assert_eq!(err.code, "chat_already_exists".to_string());
                assert_eq!(err.message, "A chat from user id '2' to user id '3' already exists, chat id: '2' ".to_string());
            }
        };
        
    }
    #[test]
    fn get_chats_returns_empty_if_none_exist() {
        let mut repository = ChatRepository::new();
        repository.insert(1, 2);
        repository.insert(2, 3);
        repository.insert(3, 1);
        let actual = repository.get_chats_for_user(4);
        assert_eq!(0, actual.len());
    }
    #[test]
    fn get_chats_returns_correctly() {
        let mut repository = ChatRepository::new();
        repository.insert(1, 2);
        repository.insert(2, 3);
        repository.insert(3, 1);
        let actual = repository.get_chats_for_user(3);
        assert_eq!(2, actual.len());
    }
}