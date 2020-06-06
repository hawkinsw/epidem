extern crate epidem;

use epidem::epidem::model::IsModel;
use epidem::epidem::sir::SIR;

struct Sir {
    // Rate of susceptibles -> infecteds
    lambda: f64,
    // Rate of infecteds -> recovereds
    r: f64,
    // Basic infection rate
    r_not: f64,
    // the number of to execute the model
    iterations: i64,
    // Current number of susceptible people in the population.
    susceptibles: f64,
    // Current number of infected people in the population.
    infecteds: f64,
    // Current number of recovered people in the population.
    recovereds: f64,
}

impl Sir {
    fn new(
        lambda: f64,
        infection_duration: f64,
        r_not: f64,
        iterations: i64,
        population: f64,
    ) -> Self {
        Sir {
            lambda,
            r: 1.0 / infection_duration,
            r_not,
            iterations,
            susceptibles: population,
            infecteds: 0.0,
            recovereds: 0.0,
        }
    }

    fn next(&mut self) {
        let becoming_infected = self.lambda * self.susceptibles;
        let becoming_recovered = self.r * self.infecteds;

        let next_susceptibles = self.susceptibles - becoming_infected;
        let next_infecteds = self.infecteds + becoming_infected - becoming_recovered;
        let next_recovereds = self.recovereds + becoming_recovered;

        self.susceptibles = next_susceptibles;
        self.infecteds = next_infecteds;
        self.recovereds = next_recovereds;
    }

    fn run(&mut self) {
        let mut iter = 0;
        while iter < self.iterations {
            self.next();
            println!(
                "{}, {}, {}, {}",
                iter, self.susceptibles, self.infecteds, self.recovereds
            );
            iter += 1;
        }
    }
}

fn main() {
    let mut m = SIR::new(&"Testing SIR", 100, 0, 0);
    //let mut model = Sir::new(0.05, 6.0, 1.0, 100, 100.0);
    for x in 1..10 {
        m.next();
    }
}
