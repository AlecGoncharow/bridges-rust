use super::element::Element;
use super::link::Link;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct LinkedList<T: Default> {
    visual: String,
    pub dims: Vec<i8>,
    nodes: Vec<Element<T>>,
    links: Vec<Link>,
    #[serde(skip_serializing, skip_deserializing)]
    head: Option<Rc<RefCell<Element<T>>>>,
    #[serde(skip_serializing, skip_deserializing)]
    tail: Option<Rc<RefCell<Element<T>>>>,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            visual: String::from("DoublyLinkedList"),
            dims: vec![0, 0, 0],
            nodes: vec![],
            links: vec![],
            head: None,
            tail: None,
        }
    }
}
