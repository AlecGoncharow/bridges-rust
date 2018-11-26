use super::element::Element;
use super::CloneDefault;

#[derive(Serialize, Deserialize)]
pub struct Array<T: CloneDefault> {
    visual: String,
    pub dims: Vec<i8>,
    pub nodes: Vec<Element<T>>,
}

impl<T: CloneDefault> Array<T> {
    pub fn new() -> Self {
        Array {
            visual: String::from("Array"),
            dims: vec![0, 0, 0],
            nodes: vec![],
        }
    }
}

pub fn new<T: CloneDefault>() -> Array<T> {
    Array::<T>::new()
}
