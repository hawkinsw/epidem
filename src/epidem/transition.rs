use crate::epidem::compartment::IsCompartment;
pub struct Transition {}

pub trait IsTransition {
    fn transition<T: IsCompartment, V: IsCompartment>(&self, from: &mut T, to: &mut V);
}

impl IsTransition for Transition {
    fn transition<T: IsCompartment, V: IsCompartment>(&self, from: &mut T, to: &mut V) {}
}

pub struct ConstantRate {
    basic: Transition,
    rate: f64,
}

impl ConstantRate {
    pub fn new(rate: f64) -> Self {
        Self {
            basic: Transition {},
            rate: rate,
        }
    }
}

impl IsTransition for ConstantRate {
    fn transition<T: IsCompartment, V: IsCompartment>(&self, from: &mut T, to: &mut V) {
        let becoming_to = from.get_current_value() * self.rate;
        //println!("becoming_to: {}", becoming_to);
        to.adjust_next_value(becoming_to);
        from.adjust_next_value(-1.0 * becoming_to);
    }
}
pub struct DynamicRate {
    basic: Transition,
    rate: f64,
}

impl IsTransition for DynamicRate {
    fn transition<T: IsCompartment, V: IsCompartment>(&self, from: &mut T, to: &mut V) {
        let becoming_to = from.get_current_value() * self.rate;
        //println!("becoming_to: {}", becoming_to);
        to.adjust_next_value(becoming_to);
        from.adjust_next_value(-1.0 * becoming_to);
    }
}

impl DynamicRate {
    pub fn new(initial_rate: f64) -> Self {
        Self {
            basic: Transition {},
            rate: initial_rate,
        }
    }

    pub fn set_rate(&mut self, new_rate: f64) {
        self.rate = new_rate;
    }
}
