extern crate epidem;

use crate::epidem::epidem::compartment::IsCompartment;
use epidem::epidem::model::IsModel;
use epidem::epidem::sir::SIR;

fn main() {
    let mut m = SIR::new(&"Testing SIR", 100, 1, 0);
    //let mut model = Sir::new(0.05, 6.0, 1.0, 100, 100.0);
    for x in 1..100 {
        m.next();
        println!(
            "{}, {}, {}, {}",
            x,
            m.susceptible.get_current_value(),
            m.infected.get_current_value(),
            m.recovered.get_current_value()
        );
    }
}
