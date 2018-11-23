use super::element::Element;
use super::link::Link;

#[derive(Serialize, Deserialize)]
pub struct LinkedList<T: Default> {
    visual: String,
    pub dims: Vec<i8>,
    nodes: Vec<Element<T>>,
    links: Vec<Link>,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            visual: String::from("DoublyLinkedList"),
            dims: vec![0, 0, 0],
            nodes: vec![],
            links: vec![],
        }
    }
}
