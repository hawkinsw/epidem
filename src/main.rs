extern crate epidem;

use crate::epidem::epidem::compartment::IsCompartment;
use epidem::epidem::model::IsModel;
use epidem::epidem::sirs::SIRS;

fn main() {
    let mut m = SIRS::new(&"Testing SIRS", 100, 1, 0);
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
