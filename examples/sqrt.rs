use rand::prelude::*;
use rand::distributions::Uniform;
use genalg::population::*;
use genalg::genalg::GenAlg;

#[derive(Clone, Copy, Debug)]
struct Cand(f32);


impl Candidate for Cand {
    fn fitness(&self) -> f32 {
        (25.0 - self.0.powf(2.0)).abs()
    }

    fn mutate(self) -> Self {
        Cand(self.0 * 1.1)
    }

    fn random<R: Rng>(rng: &mut R) -> Self {
        let d = Uniform::new_inclusive(1.0, 30.0);

        Cand(rng.sample(d))
    }

    fn reproduce(&self, other: &Self) -> Self {
        Cand((self.0 + other.0) / 2.0)
    }
}


fn main() {
    let genalg = GenAlg::<Cand>::new(5, 5, 5);
    let mut solution = Cand(0.0);

    for pop in genalg.take(1000) {
        println!("{:?}", pop);

        let c: Vec<_> = pop.iter().take(1).copied().collect();

        solution = c[0];
    }

    println!("Cand(sqrt(25)) ~= {:?}", solution);
}
