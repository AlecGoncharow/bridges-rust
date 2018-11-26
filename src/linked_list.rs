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
        if let Some(ref mut _next) = self.head {
            self.nodes.push(node.clone());
            let len = self.nodes.len() as u32;
            self.links.push(link::new(len - 2, len - 1));
            self.links.push(link::new(len - 1, len - 2));
            let mut node_clone = node.clone();
            let mut tail_clone = self.tail.clone();
            let new = Element::append(&mut tail_clone.unwrap(), &mut node_clone);
            self.tail = new;
        } else {
            self.nodes.push(node.clone());
            let new = Rc::new(RefCell::new(node));
            self.head = Some(new.clone());
            self.tail = Some(new);
        }
    }
}

pub fn new<T: CloneDefault>() -> LinkedList<T> {
    LinkedList::<T>::new()
}
