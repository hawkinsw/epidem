use crate::epidem::compartment::Compartment;
use crate::epidem::compartment::IsCompartment;
use crate::epidem::model::BasicModel;
use crate::epidem::model::IsModel;
use crate::epidem::transition::{ConstantRate, DynamicRate, IsTransition};

pub struct SIR {
    basic: BasicModel,
    pub susceptible: Compartment,
    pub infected: Compartment,
    pub recovered: Compartment,
    population: Compartment,
    dead: Compartment,
    beta: f64,
    s_to_i: DynamicRate,
    i_to_r: ConstantRate,
    birthrate: ConstantRate,
    deathrate: ConstantRate,
}

impl SIR {
    pub fn new(name: &str, susceptibles: i64, infecteds: i64, recovereds: i64) -> Self {
        let susceptible = Compartment::new("Susceptible".to_string(), susceptibles);
        let infected = Compartment::new("Infected".to_string(), infecteds);
        let recovered = Compartment::new("Recovered".to_string(), recovereds);
        let population = Compartment::new(
            "Population".to_string(),
            susceptibles + infecteds + recovereds,
        );
        let dead = Compartment::new("Dead".to_string(), 0);

        let beta = 0.005;

        let s_to_i = DynamicRate::new(beta);
        let i_to_r = ConstantRate::new(0.075);
        let birthrate = ConstantRate::new(1.0 / (365.0 * 70.0));
        let deathrate = ConstantRate::new(1.0 / (365.0 * 70.0));

        Self {
            basic: BasicModel {
                name: name.to_string(),
            },
            susceptible,
            infected,
            recovered,
            population,
            dead,
            beta,
            s_to_i,
            i_to_r,
            birthrate,
            deathrate,
        }
    }
}

impl IsModel for SIR {
    fn name(&self) -> std::string::String {
        self.basic.name.clone()
    }
    fn next(&mut self) {
        //self.population.set_current_value(
        //self.susceptible.get_current_value()
        //+ self.infected.get_current_value()
        //+ self.recovered.get_current_value(),
        //);
        //self.birthrate.transition(&mut self.population, &mut self.susceptible);
        //self.deathrate.transition(&mut self.susceptible, &mut self.dead);
        //self.deathrate.transition(&mut self.infected, &mut self.dead);
        //self.deathrate.transition(&mut self.recovered, &mut self.dead);

        self.s_to_i
            .set_rate(self.beta * self.infected.get_current_value());
        self.s_to_i
            .transition(&mut self.susceptible, &mut self.infected);
        self.i_to_r
            .transition(&mut self.infected, &mut self.recovered);

        self.infected.next();
        self.susceptible.next();
        self.recovered.next();
    }
}
