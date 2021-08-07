use crate::population::*;
use rand::prelude::*;

fn select<'a, C: Candidate + Copy>(p: &'a mut Population<C>) -> Population<C> {
    p.fittest().take(30).collect()
}


pub struct GenAlg<C: Candidate> {
    random_size: usize,
    fittest_size: usize,
    fittest_mutated_size: usize,
    older: Population<C>
}


impl<C: Candidate + Copy> GenAlg<C> {
    pub fn new(random_size: usize, fittest_size: usize, fittest_mutated_size: usize) -> Self {
        Self{
            random_size: random_size,
            fittest_size: fittest_size,
            fittest_mutated_size: fittest_mutated_size,
            older: Population::new(30)
        }
    }

    pub fn solution(&mut self) -> C {
        let c : Vec<_> = self.older.fittest().take(1).collect();

        c[0]
    }
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
