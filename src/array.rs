#[derive(Serialize, Deserialize)]
pub struct Array<T> {
    dims: Vec<i8>,
    nodes: Vec<T>,
}
