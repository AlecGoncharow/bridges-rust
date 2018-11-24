#[derive(Serialize, Deserialize)]
pub struct Link {
    pub color: Vec<f32>,
    pub thickness: u32,
    pub weight: u32,
    pub source: u32,
    pub target: u32,
}

impl Link {
    pub fn new(u32 source, u32 target) -> Self {
        Link {
            color: vec![0.0, 0.0, 0.0, 1.0],
            thickness: 1,
            weight: 1,
            source,
            target,
        }
    }
}

pub fn new(u32 source, u32 target) -> Link {
    Link::new(source, target);
}
