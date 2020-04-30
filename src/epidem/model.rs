pub trait IsModel {
    fn name(&self) -> String;
    fn next(&self);
}

struct BasicModel {
    pub name: String,
    pub compartments: Vec<Compartment>,
}

pub struct Model {
    basic: BasicModel,
}

impl Model {
    pub fn new(name: &str, compartments: Vec<Compartment>) -> Self {
        Self{basic: BasicModel{name: name.to_string(), compartments}}
    }
}

impl IsModel for Model {
    fn name(&self) -> String {
        return self.basic.name.clone();
    }
    fn next(&self) {
    }
}

pub struct Compartment {
    name: String,
}
