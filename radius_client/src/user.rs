#[derive(Debug)]
pub struct Attribute {
    pub vendor: u32,
    pub subtype: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct User {
    attributes: Vec<Attribute>,
}

impl User {
    pub fn new() -> Self {
        User { attributes: vec![] }
    }

    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }
}
