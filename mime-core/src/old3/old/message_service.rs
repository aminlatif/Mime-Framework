pub trait MessageService {
    fn send_message(&self, message: &str);
}
