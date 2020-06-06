use crate::epidem::compartment::Compartment;
use crate::epidem::compartment::IsCompartment;
use crate::epidem::model::BasicModel;
use crate::epidem::model::IsModel;
use crate::epidem::transition::{ConstantRate, IsTransition};

pub struct SIR {
    basic: BasicModel,
    susceptible: Compartment,
    infected: Compartment,
    recovered: Compartment,
    s_to_i: ConstantRate,
    i_to_r: ConstantRate,
}

impl SIR {
    pub fn new(name: &str, susceptibles: i64, infecteds: i64, recovereds: i64) -> Self {
        let susceptible = Compartment::new("Susceptible".to_string(), susceptibles);
        let infected = Compartment::new("Infected".to_string(), infecteds);
        let recovered = Compartment::new("Recovered".to_string(), recovereds);
        let s_to_i = ConstantRate::new(5.0);
        let i_to_r = ConstantRate::new(5.0);
        Self {
            basic: BasicModel {
                name: name.to_string(),
            },
            susceptible,
            infected,
            recovered,
            s_to_i,
            i_to_r,
        }
    }
}

impl IsModel for SIR {
    fn name(&self) -> std::string::String {
        self.basic.name.clone()
    }
    fn next(&mut self) {
        self.s_to_i
            .transition(&mut self.susceptible, &mut self.infected);
        self.i_to_r
            .transition(&mut self.infected, &mut self.recovered);
    }
}
