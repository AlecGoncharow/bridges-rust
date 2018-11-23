use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Serialize, Deserialize)]
pub struct Element<T: Default> {
    pub value: T,
    pub name: String,
    pub color: Vec<f32>,
    pub size: f32,
    pub location: Vec<f32>,
    pub shape: String,
    #[serde(skip_serializing, skip_deserializing)]
    next: Option<Weak<RefCell<Element<T>>>>,
    #[serde(skip_serializing, skip_deserializing)]
    prev: Option<Rc<RefCell<Element<T>>>>,
}

impl<T: Default> Element<T> {
    pub fn new(value: T) -> Element<T> {
        Element {
            value,
            name: String::from(""),
            color: vec![0.0, 0.0, 0.0, 1.0],
            size: 10.0,
            location: vec![0.0, 0.0],
            shape: String::from("circle"),
            next: None,
            prev: None,
        }
    }
}

pub fn new<T: Default>(value: T) -> Element<T> {
    Element::<T>::new(value)
}
