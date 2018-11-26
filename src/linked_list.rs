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
    #[serde(skip_serializing, skip_deserializing)]
    list_type: ListType,
}

pub enum ListType {
    Single,
    Double,
    CircleSingle,
    CircleDouble,
}

impl Default for ListType {
    fn default() -> ListType {
        ListType::Single
    }
}

impl<T: CloneDefault> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            visual: String::from("SinglyLinkedList"),
            nodes: vec![],
            links: vec![],
            head: None,
            tail: None,
            list_type: ListType::default(),
        }
    }

    pub fn append(&mut self, node: Element<T>) {
        if !self.head.is_none() {
            self.nodes.push(node.clone());
            let len = self.nodes.len() as u32;
            match self.list_type {
                ListType::Single => {
                    self.links.push(link::new(len - 2, len - 1));
                }
                ListType::Double => {
                    self.links.push(link::new(len - 2, len - 1));
                    self.links.push(link::new(len - 1, len - 2));
                }
                ListType::CircleSingle => {
                    self.links.pop();
                    self.links.push(link::new(len - 2, len - 1));
                    self.links.push(link::new(len - 1, 0));
                }
                ListType::CircleDouble => {
                    self.links.pop();
                    self.links.pop();
                    self.links.push(link::new(len - 2, len - 1));
                    self.links.push(link::new(len - 1, len - 2));
                    self.links.push(link::new(len - 1, 0));
                    self.links.push(link::new(0, len - 1));
                }
            };
            self.tail = Some(Rc::new(RefCell::new(node)));
        } else {
            self.nodes.push(node.clone());
            self.head = Some(Rc::new(RefCell::new(node.clone())));
            self.tail = Some(Rc::new(RefCell::new(node)));
        }
    }

    pub fn set_list_type(&mut self, list_type: ListType) {
        self.visual = match list_type {
            ListType::Single => String::from("SinglyLinkedList"),
            ListType::Double => String::from("DoublyLinkedList"),
            ListType::CircleSingle => String::from("CircularDoublyLinkedList"),
            ListType::CircleDouble => String::from("CircularDoublyLinkedList"),
        };
        self.list_type = list_type;
    }
}

pub fn new<T: CloneDefault>() -> LinkedList<T> {
    LinkedList::<T>::new()
}
