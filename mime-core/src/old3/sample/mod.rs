use crate::{component::Component, container::{self, Container}};

pub trait SampleService: Component {
    fn new(container: &Container) -> Box<Self>;
    fn get_sample_string(&self) -> String;
}

pub struct SampleServiceImpl {}

impl Component for SampleServiceImpl {
    fn new (container: &Container) -> Self {
        Box<Self {}>
    }
}

impl SampleService for SampleServiceImpl {
    fn get_sample_string(&self) -> String {
        "sample string".to_string()
    }
}