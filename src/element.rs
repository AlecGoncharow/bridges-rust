use super::CloneDefault;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Serialize, Deserialize, Clone)]
pub struct Element<T: CloneDefault> {
    pub value: T,
    pub name: String,
    pub color: Vec<f32>,
    pub size: f32,
    pub location: Vec<f32>,
    pub shape: String,
    #[serde(skip_serializing, skip_deserializing)]
    next: Option<Rc<RefCell<Element<T>>>>,
    #[serde(skip_serializing, skip_deserializing)]
    prev: Option<Weak<RefCell<Element<T>>>>,
}

impl<T: CloneDefault> Element<T> {
    pub fn new(value: T) -> Self {
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

    pub fn append(
        current: &mut Rc<RefCell<Element<T>>>,
        new: &mut Element<T>,
    ) -> Option<Rc<RefCell<Element<T>>>> {
        new.prev = Some(Rc::downgrade(&current));
        let rc = Rc::new(RefCell::new(new.clone()));
        current.borrow_mut().next = Some(rc.clone());
        Some(rc.clone())
    }
}

pub fn new<T: CloneDefault>(value: T) -> Element<T> {
    Element::new(value)
}
