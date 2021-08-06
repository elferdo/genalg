use rand::prelude::*;
use rand::distributions::Uniform;
use rand::seq::IteratorRandom;
use genalg::population::*;


#[derive(Clone, Copy, Debug)]
struct Cand(f32);


impl Candidate for Cand {
    fn fitness(&self) -> f32 {
        (25.0 - self.0.powf(2.0)).abs()
    }

    fn mutate(self) -> Self {
        Cand(self.0 * 1.1)
    }

    fn random() -> Self {
        let d = Uniform::new_inclusive(1.0, 30.0);
        let mut rng = thread_rng();

        Cand((&mut rng).sample(d))
    }

    fn reproduce(&self, other: &Self) -> Self {
        Cand((self.0 + other.0) / 2.0)
    }
}


fn select<'a, C: Candidate + Copy>(p: &'a mut Population<C>) -> Population<C> {
    p.fittest().take(30).collect()
}


struct GenAlg<C: Candidate> {
    random_size: usize,
    fittest_size: usize,
    fittest_mutated_size: usize,
    older: Population<C>
}


impl<C: Candidate + Copy> Iterator for GenAlg<C> {
    type Item = Population<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();

        let new_size = self.older.size();
        let mut result = Population::<C>::new(self.random_size);

        let mut selected = select(&mut self.older);

        result.extend(selected.fittest().take(self.fittest_size));
        result.extend(selected.iter().choose_multiple(&mut rng, self.fittest_mutated_size).iter().map(|x| x.mutate()));

        for _ in (self.random_size + self.fittest_size + self.fittest_mutated_size + 1)..=new_size {
            let older_candidate = self.older.iter().choose(&mut rng).unwrap();
            let selected_candidate = selected.iter().choose(&mut rng).unwrap();

            result.push(older_candidate.reproduce(&selected_candidate));
        }

        self.older = result.clone();

        Some(result)
    }
}


fn main() {
    let genalg = GenAlg::<Cand>{random_size: 5, fittest_size: 5, fittest_mutated_size: 5, older: Population::new(30)};

    for pop in genalg.take(1000) {
        println!("{:?}", pop);
    }
}
