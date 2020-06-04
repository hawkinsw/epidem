pub struct Compartment {
    name: String,
    current_value: f64,
    next_value: f64,
}

pub trait IsCompartment {
    fn get_current_value(&mut self) -> f64;
    fn set_current_value(&mut self, current_value: f64) -> f64;
    fn adjust_next_value(&mut self, adjustment: f64);
    fn new(name: String) -> Self;
}

impl IsCompartment for Compartment {
    fn get_current_value(&mut self) -> f64 {
        self.current_value
    }

    fn set_current_value(&mut self, current_value: f64) -> f64 {
        self.current_value = current_value;
        self.current_value
    }

    fn adjust_next_value(&mut self, adjustment: f64) {
        self.next_value += adjustment;
    }

    fn new(name: String) -> Self {
        Self{name: name, current_value: 0.0, next_value: 0.0}
    }
}