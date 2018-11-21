#[derive(Serialize, Deserialize)]
pub struct Element<T> {
    pub value: T,
    pub name: String,
    pub color: Vec<f32>,
    pub size: f32,
    pub location: Vec<f32>,
    pub shape: String,
}

impl<T: Default> Element<T> {
    pub fn new() -> Element<T> {
        Element {
            value: T::default(),
            name: String::from(""),
            color: vec![0.0, 0.0, 0.0, 1.0],
            size: 10.0,
            location: vec![0.0, 0.0],
            shape: String::from("circle"),
        }
    }
}

pub fn new<T: Default>() -> Element<T> {
    Element::<T>::new()
}
