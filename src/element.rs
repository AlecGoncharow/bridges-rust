use super::CloneDefault;

#[derive(Serialize, Deserialize, Clone)]
pub struct Element<T: CloneDefault> {
    pub value: T,
    pub name: String,
    pub color: Vec<f32>,
    pub size: f32,
    pub location: Vec<f32>,
    pub shape: String,
}

impl<T: CloneDefault> Element<T> {
    pub fn new(value: T) -> Self {
        Element {
            value,
            name: String::from(""),
            color: vec![0.0, 0.0, 0.0, 1.0],
            size: 10.0,
            location: vec![0.0, 0.0],
            shape: String::from("circle"),
        }
    }
}

pub fn new<T: CloneDefault>(value: T) -> Element<T> {
    Element::new(value)
}
