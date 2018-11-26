use super::element::Element;
use super::link;
use super::link::Link;
use super::CloneDefault;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct LinkedList<T: CloneDefault> {
    visual: String,
    nodes: Vec<Element<T>>,
    links: Vec<Link>,
    #[serde(skip_serializing, skip_deserializing)]
    head: Option<Rc<RefCell<Element<T>>>>,
    #[serde(skip_serializing, skip_deserializing)]
    tail: Option<Rc<RefCell<Element<T>>>>,
}

impl<T: CloneDefault> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            visual: String::from("DoublyLinkedList"),
            nodes: vec![],
            links: vec![],
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, node: Element<T>) {
        if !self.head.is_none() {
            self.nodes.push(node.clone());
            let len = self.nodes.len() as u32;
            self.links.push(link::new(len - 2, len - 1));
            self.links.push(link::new(len - 1, len - 2));
            self.tail = Some(Rc::new(RefCell::new(node)));
        } else {
            self.nodes.push(node.clone());
            self.head = Some(Rc::new(RefCell::new(node.clone())));
            self.tail = Some(Rc::new(RefCell::new(node)));
        }
    }
}

pub fn new<T: CloneDefault>() -> LinkedList<T> {
    LinkedList::<T>::new()
}
