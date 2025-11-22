use super::message_service::MessageService;

pub struct EmailMessageService {
    username: String,
    password: String,
}

impl EmailMessageService {
    pub fn new(username: String, password: String) -> Self {
        EmailMessageService { username, password }
    }
}

impl MessageService for EmailMessageService {
    fn send_message(&self, message: &str) {
        println!("Sending message: {}", message);
        // Logic to send the message using the email service credentials
    }
}
