pub trait NotifierService {
    fn notify(&self, msg: &str);
}
