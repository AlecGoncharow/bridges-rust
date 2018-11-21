use super::element::Element;

#[derive(Serialize, Deserialize)]
pub struct Array<T> {
    visual: String,
    pub dims: Vec<i8>,
    pub nodes: Vec<Element<T>>,
}

impl<T: Default> Array<T> {
    pub fn new() -> Array<T> {
        Array {
            visual: String::from("Array"),
            dims: vec![0, 0, 0],
            nodes: vec![],
        }
    }
}

pub fn new<T: Default>() -> Array<T> {
    Array::<T>::new()
}
