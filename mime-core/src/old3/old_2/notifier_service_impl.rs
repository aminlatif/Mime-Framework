use crate::notifier_service::NotifierService;

struct EmailNotifier;

impl NotifierService for EmailNotifier {
    fn notify(&self, msg: &str) {
        println!("Email: {}", msg);
    }
}
