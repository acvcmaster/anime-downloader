#[derive(Debug)]
pub struct Metadata {
    pub name: String,
    pub episode: i32,
}

impl Metadata {
    pub fn new(name: String, episode: i32) -> Self {
        Self { name, episode }
    }
}
