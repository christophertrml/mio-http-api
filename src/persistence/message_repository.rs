use crate::models::message::Message;
use crate::models::errors::ApiError;
use std::collections::HashMap;

pub struct MessageRepository {
    chat_messages: HashMap<u64, Vec<Message>>,
}

impl MessageRepository {
    pub fn new() -> MessageRepository {
        MessageRepository {
            chat_messages: HashMap::new()
        }
    }

    pub fn add_message(&mut self, message: Message, chat_id: u64) {
        match self.chat_messages.get_mut(&chat_id) {
            Some(messages) => messages.push(message),
            None => {
                let mut vec = Vec::new();
                vec.push(message);
                self.chat_messages.insert(chat_id, vec);
            }
        }
    }

    pub fn get_messages_for_chat_id(&self, chat_id: u64) -> Result<&Vec<Message>, ApiError> {
        match self.chat_messages.get(&chat_id) {
            Some(messages) => Ok(messages),
            None => Err(ApiError::chat_does_not_exist(chat_id))
        }
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    
    #[test]
    fn get_messages_returns_successfully() {
        let mut repository = MessageRepository::new();
        let test_message_1 = Message {
            source_user_id: 1,
            destination_user_id: 2,
            timestamp: 123,
            message: "test".to_string()
        };
        let test_message_2 = Message {
            source_user_id: 1,
            destination_user_id: 2,
            timestamp: 123,
            message: "test".to_string()
        };
        let test_message_3 = Message {
            source_user_id: 1,
            destination_user_id: 2,
            timestamp: 123,
            message: "test".to_string()
        };

        repository.add_message(test_message_1, 1);
        repository.add_message(test_message_2, 1);
        repository.add_message(test_message_3, 2);
        
        match repository.get_messages_for_chat_id(1) {
            Ok(messages) => assert_eq!(2, messages.len()),
            Err(e) => assert!(false, e.message) 
        };
    }

    #[test]
    fn get_messages_returns_error_when_chat_does_not_exist() {
        let mut repository = MessageRepository::new();
        let test_message_1 = Message {
            source_user_id: 1,
            destination_user_id: 2,
            timestamp: 123,
            message: "test".to_string()
        };
        let test_message_2 = Message {
            source_user_id: 1,
            destination_user_id: 2,
            timestamp: 123,
            message: "test".to_string()
        };
        let test_message_3 = Message {
            source_user_id: 1,
            destination_user_id: 2,
            timestamp: 123,
            message: "test".to_string()
        };

        repository.add_message(test_message_1, 1);
        repository.add_message(test_message_2, 1);
        repository.add_message(test_message_3, 2);
        
        
        match repository.get_messages_for_chat_id(3) {
            Ok(_) => assert!(false, "messages should not have been recorded for chat_id 3"),
            Err(e) => assert_eq!("chat_does_not_exist".to_string(), e.code) 
        };
    }
}
