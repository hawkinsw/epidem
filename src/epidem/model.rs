pub trait IsModel {
    fn name(&self) -> String;
    fn next(&mut self);
}

pub struct BasicModel {
    pub name: String,
}

pub struct Model {
    basic: BasicModel,
}

impl Model {
    pub fn new(name: &str) -> Self {
        Self {
            basic: BasicModel {
                name: name.to_string(),
            },
        }
    }
}

impl IsModel for Model {
    fn name(&self) -> String {
        return self.basic.name.clone();
    }
    fn next(&mut self) {}
}
