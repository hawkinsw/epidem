pub struct Compartment {
    name: String,
    current_value: f64,
    next_value: f64,
}

pub trait IsCompartment {
    fn get_current_value(&mut self) -> f64;
    fn set_current_value(&mut self, current_value: f64) -> f64;
    fn adjust_next_value(&mut self, adjustment: f64);
    fn next(&mut self);
    fn new(name: String, initial_value: i64) -> Self;
}

impl IsCompartment for Compartment {
    fn get_current_value(&mut self) -> f64 {
        self.current_value
    }
    fn set_current_value(&mut self, current_value: f64) -> f64 {
        self.current_value = current_value;
        self.next_value = self.current_value;
        self.current_value
    }

    fn adjust_next_value(&mut self, adjustment: f64) {
        self.next_value += adjustment;
    }

    fn new(name: String, initial_value: i64) -> Self {
        Self {
            name: name,
            current_value: initial_value as f64,
            next_value: initial_value as f64,
        }
    }
    fn next(&mut self) {
        self.current_value = self.next_value;
    }
}
