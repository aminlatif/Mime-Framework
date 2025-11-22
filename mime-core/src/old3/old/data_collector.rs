pub trait DataCollector {
    fn collect_data(&self) -> Vec<String>;
}

impl<T: DataCollector + ?Sized> DataCollector for Box<T> {
    fn collect_data(&self) -> Vec<String> {
        T::collect_data(self)
    }
}