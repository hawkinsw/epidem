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
        to.adjust_next_value(becoming_to);
        from.adjust_next_value(-1.0 * becoming_to);
    }
}
