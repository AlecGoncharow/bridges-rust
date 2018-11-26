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

    /// Takes in an Element<T> and appends it to the list according to the type of list
    /// # Example
    /// ```
    /// use bridges::linked_list::{LinkedList, ListType};
    /// use bridges::element::Element;
    ///
    /// let mut my_list: LinkedList<i32> = LinkedList::new();
    /// my_list.set_list_type(ListType::Double);
    /// let mut my_element = Element::new(0);
    /// // Style options here
    /// my_list.append(my_element);
    ///
    /// ```
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

    /// Returns the link created from appending elements to the list
    /// # Example
    /// ```
    /// use bridges::linked_list::LinkedList;
    /// let mut my_list: LinkedList<i32> = LinkedList::new();
    /// let my_link = my_list.get_link(1, 2);
    /// ```
    pub fn get_link(&mut self, source: T, target: T) -> Option<&mut Link> {
        let s = match self.nodes.iter().position(|ref e| e.value == source) {
            Some(e) => e as u32,
            None => return None,
        };
        let t = match self.nodes.iter().position(|ref e| e.value == target) {
            Some(e) => e as u32,
            None => return None,
        };

        self.links
            .iter_mut()
            .find(|ref mut l| l.source == s && l.target == t)
    }
}

pub fn new<T: CloneDefault>() -> LinkedList<T> {
    LinkedList::<T>::new()
}
