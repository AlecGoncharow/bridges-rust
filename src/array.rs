#[derive(Serialize, Deserialize)]
pub struct Array<T> {
    visual: String,
    dims: Vec<i8>,
    nodes: Vec<T>,
}

impl<T> Array<T> {
    pub fn new() -> Array<T> {
        Array {
            visual: String::from("Array"),
            dims: vec![0, 0, 0],
            nodes: vec![],
        }
    }
}
